//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! function call frame
//!    Frame
//!    apply
//!    frame_push
//!    frame_pop
//!    frame_ref
use crate::{
    core::{
        apply::Apply as _,
        dynamic::Dynamic,
        env::Env,
        exception::{self, Condition, Exception},
        namespace::Namespace,
        types::{Tag, Type},
    },
    types::{
        cons::Cons, fixnum::Fixnum, function::Function, struct_::Struct, symbol::Symbol,
        vector::Vector,
    },
};

#[cfg(feature = "prof")]
use crate::features::{feature::Feature, prof::Prof};

use {futures::executor::block_on, futures_locks::RwLock};

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
        match tag.type_of() {
            Type::Struct => {
                let stype = Struct::stype(env, tag);
                let frame = Struct::vector(env, tag);

                let func = Vector::ref_(env, frame, 0).unwrap();

                match func.type_of() {
                    Type::Function => {
                        if !stype.eq_(&Symbol::keyword("frame")) {
                            panic!()
                        }

                        Frame {
                            argv: Vector::iter(env, frame).skip(1).collect::<Vec<Tag>>(),
                            func,
                            value: Tag::nil(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    // frame stacks
    fn frame_stack_push(self, env: &Env) {
        let id = self.func.as_u64();

        let mut stack_ref = block_on(env.lexical.write());

        if let std::collections::hash_map::Entry::Vacant(e) = stack_ref.entry(id) {
            e.insert(RwLock::new(vec![self]));
        } else {
            let mut vec_ref = block_on(stack_ref[&id].write());

            vec_ref.push(self);
        }
    }

    fn frame_stack_pop(env: &Env, id: Tag) {
        let stack_ref = block_on(env.lexical.read());
        let mut vec_ref = block_on(stack_ref[&id.as_u64()].write());

        vec_ref.pop();
    }

    fn frame_stack_len(env: &Env, id: Tag) -> Option<usize> {
        let stack_ref = block_on(env.lexical.read());

        if stack_ref.contains_key(&id.as_u64()) {
            let vec_ref = block_on(stack_ref[&id.as_u64()].read());

            Some(vec_ref.len())
        } else {
            None
        }
    }

    pub fn frame_stack_ref(env: &Env, id: Tag, offset: usize, argv: &mut Vec<u64>) {
        let stack_ref = block_on(env.lexical.read());
        let vec_ref = block_on(stack_ref[&id.as_u64()].read());

        argv.extend(vec_ref[offset].argv.iter().map(|value| value.as_u64()));
    }

    // frame reference
    fn frame_ref(env: &Env, id: u64, offset: usize) -> Option<Tag> {
        let stack_ref = block_on(env.lexical.read());
        let vec_ref = block_on(stack_ref[&id].read());

        Some(vec_ref[vec_ref.len() - 1].argv[offset])
    }

    // apply
    pub fn apply(mut self, env: &Env, func: Tag) -> exception::Result<Tag> {
        #[cfg(feature = "prof")]
        <Feature as Prof>::prof_event(env, func).unwrap();

        let nreqs = Fixnum::as_i64(Function::arity(env, func)) as usize;
        let nargs = self.argv.len();

        if nargs != nreqs {
            return Err(Exception::new(env, Condition::Arity, "mu:apply", func));
        }

        match func.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, func) {
                    self.apply(env, Symbol::value(env, func))
                } else {
                    Err(Exception::new(env, Condition::Unbound, "mu:apply", func))
                }
            }
            Type::Function => {
                let form = Function::form(env, func);
                let offset = Cons::cdr(env, form);

                match form.type_of() {
                    Type::Null => Ok(Tag::nil()),
                    Type::Cons => match offset.type_of() {
                        Type::Fixnum => {
                            let ns = Cons::car(env, form);
                            let ns_ref = block_on(env.ns_map.read());
                            let (_, _, ref namespace) = ns_ref[Namespace::index_of(env, ns)];

                            match namespace {
                                Namespace::Static(static_) => {
                                    let func = match static_.functions {
                                        Some(functab) => functab[Fixnum::as_i64(offset) as usize].2,
                                        None => panic!(),
                                    };

                                    drop(ns_ref);
                                    func(env, &mut self)?
                                }
                                _ => panic!(),
                            }

                            Ok(self.value)
                        }
                        Type::Null | Type::Cons => {
                            let mut value = Tag::nil();
                            let offset = Self::frame_stack_len(env, self.func).unwrap_or(0);

                            Dynamic::dynamic_push(env, self.func, offset);
                            self.frame_stack_push(env);

                            for cons in Cons::iter(env, form) {
                                value = env.eval(Cons::car(env, cons))?;
                            }

                            Self::frame_stack_pop(env, func);
                            Dynamic::dynamic_pop(env);

                            Ok(value)
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            _ => Err(Exception::new(env, Condition::Type, "mu:apply", func)),
        }
    }
}

pub trait CoreFunction {
    fn mu_frame_pop(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frame_push(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frame_ref(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_frames(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Frame {
    fn mu_frames(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frames_ref = block_on(env.dynamic.dynamic.read());
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

        env.fp_argv_check("mu:frame-pop", &[Type::Function], fp)?;

        Self::frame_stack_pop(env, fp.value);

        Ok(())
    }

    fn mu_frame_push(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.fp_argv_check("mu:frame-push", &[Type::Cons], fp)?;

        let func = Cons::car(env, fp.argv[0]);
        if func.type_of() != Type::Function {
            return Err(Exception::new(env, Condition::Type, "mu:%frame-ref", func));
        }

        let av = Cons::cdr(env, fp.argv[0]);
        if av.type_of() != Type::Vector {
            return Err(Exception::new(env, Condition::Type, "mu:%frame-ref", av));
        }

        let argv = Vector::iter(env, av).collect::<Vec<Tag>>();

        let value = Tag::nil();

        Frame { func, argv, value }.frame_stack_push(env);

        Ok(())
    }

    fn mu_frame_ref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        env.fp_argv_check("mu:%frame-ref", &[Type::Function, Type::Fixnum], fp)?;

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
