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
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Core as _, Exception},
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        function::Function,
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::VectorIter,
        vector::{Core as _, Vector},
    },
    LIB,
};
use {futures::executor::block_on, futures_locks::RwLock};

pub struct Frame {
    pub func: Tag,
    pub argv: Vec<Tag>,
    pub value: Tag,
}

impl Frame {
    fn to_tag(&self, env: &Env) -> Tag {
        let vec = self.argv.to_vec();

        Struct::new(env, "frame", vec).evict(env)
    }

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
                            func,
                            argv: VectorIter::new(env, frame).skip(1).collect::<Vec<Tag>>(),
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
        Exception::on_signal(env)?;

        match func.type_of() {
            Type::Symbol => {
                if Symbol::is_bound(env, func) {
                    self.apply(env, Symbol::value(env, func))
                } else {
                    Err(Exception::new(env, Condition::Unbound, "mu:apply", func))
                }
            }
            Type::Function => match Function::form(env, func).type_of() {
                Type::Null => Ok(Tag::nil()),
                Type::Vector => {
                    let nreqs = Fixnum::as_i64(Function::arity(env, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::new(env, Condition::Arity, "mu:apply", func));
                    }

                    let offset =
                        Fixnum::as_i64(Vector::ref_(env, Function::form(env, func), 2).unwrap());

                    let functions_ref = block_on(LIB.functions.read());
                    functions_ref[offset as usize](env, &mut self)?;

                    Ok(self.value)
                }
                Type::Cons => {
                    let nreqs = Fixnum::as_i64(Function::arity(env, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::new(env, Condition::Arity, "mu:apply", func));
                    }

                    let mut value = Tag::nil();
                    let offset = Self::frame_stack_len(env, self.func).unwrap_or(0);

                    env.dynamic_push(self.func, offset);
                    self.frame_stack_push(env);

                    for cons in Cons::iter(env, Function::form(env, func)) {
                        value = env.eval(Cons::car(env, cons))?;
                    }

                    Self::frame_stack_pop(env, func);
                    env.dynamic_pop();

                    Ok(value)
                }
                _ => Err(Exception::new(env, Condition::Type, "mu:apply", func)),
            },
            _ => Err(Exception::new(env, Condition::Type, "mu:apply", func)),
        }
    }
}

pub trait CoreFunction {
    fn mu_fr_pop(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fr_push(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fr_ref(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Frame {
    fn mu_fr_pop(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];

        env.fp_argv_check("mu:frame-pop", &[Type::Function], fp)?;

        Self::frame_stack_pop(env, fp.value);

        Ok(())
    }

    fn mu_fr_push(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];

        env.fp_argv_check("mu:frame-push", &[Type::Vector], fp)?;

        Self::from_tag(env, fp.value).frame_stack_push(env);

        Ok(())
    }

    fn mu_fr_ref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        env.fp_argv_check("mu:frame-ref", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Frame::frame_ref(
            env,
            Fixnum::as_i64(frame) as u64,
            Fixnum::as_i64(offset) as usize,
        ) {
            Some(tag) => tag,
            None => return Err(Exception::new(env, Condition::Type, "mu:frame-ref", frame)),
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
