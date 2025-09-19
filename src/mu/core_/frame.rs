//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// function call frame
use {
    crate::{
        core_::{
            apply::Apply as _,
            dynamic::Dynamic,
            env::Env,
            exception::{self, Condition, Exception},
            tag::Tag,
            type_::Type,
        },
        types::{
            async_::Async, cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct,
            symbol::Symbol, vector::Vector,
        },
    },
    futures_lite::future::block_on,
};

#[cfg(feature = "prof")]
use crate::features::{feature::Feature, prof::Prof};

pub struct Frame {
    pub argv: Vec<Tag>,
    pub func: Tag,
    pub value: Tag,
}

impl Frame {
    #[allow(dead_code)]
    fn to_tag(&self, env: &Env) -> Tag {
        let vec = self.argv.clone();

        Struct::new(env, "frame", vec).evict(env)
    }

    #[allow(dead_code)]
    fn from_tag(env: &Env, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Struct);

        let stype = Struct::stype(env, tag);
        let frame = Struct::vector(env, tag);
        let func = Vector::ref_(env, frame, 0).unwrap();

        assert_eq!(func.type_of(), Type::Function);
        assert!(stype.eq_(&Symbol::keyword("frame")));

        Frame {
            argv: Vector::iter(env, frame).skip(1).collect::<Vec<Tag>>(),
            func,
            value: Tag::nil(),
        }
    }

    // frame stacks
    fn frame_stack_push(self, env: &Env) {
        let id = self.func.as_u64();
        let mut lexical_ref = block_on(env.lexical.write());

        if let std::collections::hash_map::Entry::Vacant(e) = lexical_ref.entry(id) {
            e.insert(vec![self]);
        } else {
            let vec_ref = lexical_ref.get_mut(&id);

            vec_ref.expect("").push(self);
        }
    }

    fn frame_stack_pop(env: &Env, id: Tag) {
        let mut lexical_ref = block_on(env.lexical.write());

        lexical_ref.get_mut(&id.as_u64()).expect("").pop();
    }

    fn frame_stack_len(env: &Env, id: Tag) -> Option<usize> {
        let lexical_ref = block_on(env.lexical.read());

        if lexical_ref.contains_key(&id.as_u64()) {
            Some(lexical_ref[&id.as_u64()].len())
        } else {
            None
        }
    }

    pub fn frame_stack_ref(env: &Env, id: Tag, offset: usize, argv: &mut Vec<u64>) {
        let lexical_ref = block_on(env.lexical.read());

        argv.extend(
            lexical_ref[&id.as_u64()][offset]
                .argv
                .iter()
                .map(|value| value.as_u64()),
        );
    }

    // frame reference
    fn frame_ref(env: &Env, id: u64, offset: usize) -> Option<Tag> {
        let lexical_ref = block_on(env.lexical.read());
        let frame = lexical_ref[&id].last().unwrap();

        Some(frame.argv[offset])
    }

    // apply
    pub fn apply(mut self, env: &Env, func: Tag) -> exception::Result<Tag> {
        #[cfg(feature = "prof")]
        <Feature as Prof>::prof_event(env, func).unwrap();

        let (arity, form) = Function::destruct(env, func);

        let nreqs = Fixnum::as_i64(arity) as usize;
        let nargs = self.argv.len();

        if nargs != nreqs {
            Err(Exception::new(env, Condition::Arity, "mu:apply", func))?
        }

        match func.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, func) {
                    self.apply(env, Symbol::value(env, func))
                } else {
                    Err(Exception::new(env, Condition::Unbound, "mu:apply", func))?
                }
            }
            Type::Async => {
                let form = Async::form(env, func);
                let offset = Cons::destruct(env, form).1;

                match form.type_of() {
                    Type::Null => Ok(Tag::nil()),
                    Type::Cons => match offset.type_of() {
                        Type::Null | Type::Cons => {
                            let offset = Self::frame_stack_len(env, self.func).unwrap_or(0);

                            Dynamic::dynamic_push(env, self.func, offset);
                            self.frame_stack_push(env);

                            let value: exception::Result<Tag> = Cons::list_iter(env, form)
                                .try_fold(Tag::nil(), |_, expr| env.eval(expr));

                            Self::frame_stack_pop(env, func);
                            Dynamic::dynamic_pop(env);

                            value
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            Type::Function => {
                if let Tag::Direct(_) = func {
                    Function::staticns_deref(env, func).2 .2(env, &mut self)?;

                    return Ok(self.value);
                }

                match form.type_of() {
                    Type::Null => Ok(Tag::nil()),
                    Type::Cons => {
                        let offset = Self::frame_stack_len(env, self.func).unwrap_or(0);

                        Dynamic::dynamic_push(env, self.func, offset);
                        self.frame_stack_push(env);

                        let value: exception::Result<Tag> = Cons::list_iter(env, form)
                            .try_fold(Tag::nil(), |_, expr| env.eval(expr));

                        Self::frame_stack_pop(env, func);
                        Dynamic::dynamic_pop(env);

                        value
                    }
                    _ => panic!(),
                }
            }
            _ => Err(Exception::new(env, Condition::Type, "mu:apply", func))?,
        }
    }
}

pub trait CoreFn {
    fn mu_frame_pop(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frame_push(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frame_ref(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frames(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Frame {
    fn mu_frames(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frames_ref = block_on(env.dynamic.read());
        let mut frames = Vec::new();

        frames.extend(frames_ref.iter().map(|(func, offset)| {
            let mut argv = vec![];

            Frame::frame_stack_ref(env, (&func.to_le_bytes()).into(), *offset, &mut argv);

            let vec: Vec<Tag> = argv
                .into_iter()
                .map(|f| (&f.to_le_bytes()).into())
                .collect();

            Cons::cons(
                env,
                (&func.to_le_bytes()).into(),
                Vector::from(vec).evict(env),
            )
        }));

        fp.value = Cons::list(env, &frames);
        Ok(())
    }

    fn mu_frame_pop(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];

        env.argv_check("mu:frame-pop", &[Type::Function], fp)?;

        Self::frame_stack_pop(env, fp.value);

        Ok(())
    }

    fn mu_frame_push(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:frame-push", &[Type::Cons], fp)?;

        let (func, av) = Cons::destruct(env, fp.argv[0]);
        if func.type_of() != Type::Function {
            return Err(Exception::new(env, Condition::Type, "mu:%frame-push", func));
        }

        if av.type_of() != Type::Vector {
            return Err(Exception::new(env, Condition::Type, "mu:%frame-push", av));
        }

        let argv = Vector::iter(env, av).collect::<Vec<Tag>>();

        let value = Tag::nil();

        Frame { func, argv, value }.frame_stack_push(env);

        Ok(())
    }

    fn mu_frame_ref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        env.argv_check("mu:%frame-ref", &[Type::Function, Type::Fixnum], fp)?;

        fp.value = match Frame::frame_ref(env, frame.as_u64(), Fixnum::as_i64(offset) as usize) {
            Some(tag) => tag,
            None => return Err(Exception::new(env, Condition::Type, "mu:%frame-ref", frame)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn frame() {
        assert_eq!(2 + 2, 4);
    }
}
