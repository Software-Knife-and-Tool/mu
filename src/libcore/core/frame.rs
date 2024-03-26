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
        exception::{self, Condition, Exception},
        gc::Core as _,
        mu::{Core as _, Mu},
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::Fixnum,
        function::Function,
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vecimage::VectorIter,
        vector::{Core as _, Vector},
    },
};

use {futures::executor::block_on, futures_locks::RwLock};

pub struct Frame {
    pub func: Tag,
    pub argv: Vec<Tag>,
    pub value: Tag,
}

impl Frame {
    fn to_tag(&self, mu: &Mu) -> Tag {
        let vec = self.argv.to_vec();

        Struct::new(mu, "frame", vec).evict(mu)
    }

    fn from_tag(mu: &Mu, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Struct => {
                let stype = Struct::stype(mu, tag);
                let frame = Struct::vector(mu, tag);

                let func = Vector::ref_(mu, frame, 0).unwrap();

                match func.type_of() {
                    Type::Function => {
                        if !stype.eq_(&Symbol::keyword("frame")) {
                            panic!()
                        }

                        Frame {
                            func,
                            argv: VectorIter::new(mu, frame).skip(1).collect::<Vec<Tag>>(),
                            value: Tag::nil(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    pub fn gc_lexical(mu: &Mu) {
        let lexical_ref = block_on(mu.lexical.read());

        for frame_vec in (*lexical_ref).values() {
            let frame_vec_ref = block_on(frame_vec.read());

            for frame in frame_vec_ref.iter() {
                Mu::mark(mu, frame.func);

                for arg in &frame.argv {
                    Mu::mark(mu, *arg)
                }

                Mu::mark(mu, frame.value);
            }
        }
    }

    // frame stacks
    fn frame_stack_push(self, mu: &Mu) {
        let id = self.func.as_u64();

        let mut stack_ref = block_on(mu.lexical.write());

        if let std::collections::hash_map::Entry::Vacant(e) = stack_ref.entry(id) {
            e.insert(RwLock::new(vec![self]));
        } else {
            let mut vec_ref = block_on(stack_ref[&id].write());

            vec_ref.push(self);
        }
    }

    fn frame_stack_pop(mu: &Mu, id: Tag) {
        let stack_ref = block_on(mu.lexical.read());
        let mut vec_ref = block_on(stack_ref[&id.as_u64()].write());

        vec_ref.pop();
    }

    fn frame_stack_len(mu: &Mu, id: Tag) -> Option<usize> {
        let stack_ref = block_on(mu.lexical.read());

        if stack_ref.contains_key(&id.as_u64()) {
            let vec_ref = block_on(stack_ref[&id.as_u64()].read());

            Some(vec_ref.len())
        } else {
            None
        }
    }

    pub fn frame_stack_ref(mu: &Mu, id: Tag, offset: usize, argv: &mut Vec<u64>) {
        let stack_ref = block_on(mu.lexical.read());
        let vec_ref = block_on(stack_ref[&id.as_u64()].read());

        argv.extend(vec_ref[offset].argv.iter().map(|value| value.as_u64()));
    }

    // frame reference
    fn frame_ref(mu: &Mu, id: u64, offset: usize) -> Option<Tag> {
        let stack_ref = block_on(mu.lexical.read());
        let vec_ref = block_on(stack_ref[&id].read());

        Some(vec_ref[vec_ref.len() - 1].argv[offset])
    }

    // apply
    pub fn apply(mut self, mu: &Mu, func: Tag) -> exception::Result<Tag> {
        match func.type_of() {
            Type::Symbol => {
                if Symbol::is_unbound(mu, func) {
                    Err(Exception::new(Condition::Unbound, "apply", func))
                } else {
                    self.apply(mu, Symbol::value(mu, func))
                }
            }
            Type::Function => match Function::form(mu, func).type_of() {
                Type::Null => Ok(Tag::nil()),
                Type::Keyword | Type::Symbol => {
                    let nreqs = Fixnum::as_i64(Function::arity(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::new(Condition::Arity, "apply", func));
                    }

                    let fn_key = Function::form(mu, func);
                    let fn_ = mu.functions.borrow()[&Tag::as_u64(&fn_key)];

                    match fn_(mu, &mut self) {
                        Ok(_) => Ok(self.value),
                        Err(e) => Err(e),
                    }
                }
                Type::Cons => {
                    let nreqs = Fixnum::as_i64(Function::arity(mu, func)) as usize;
                    let nargs = self.argv.len();

                    if nargs != nreqs {
                        return Err(Exception::new(Condition::Arity, "apply", func));
                    }

                    let mut value = Tag::nil();
                    let offset = Self::frame_stack_len(mu, self.func).unwrap_or(0);

                    mu.dynamic_push(self.func, offset);
                    self.frame_stack_push(mu);

                    for cons in Cons::iter(mu, Function::form(mu, func)) {
                        value = match mu.eval(Cons::car(mu, cons)) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        };
                    }

                    Self::frame_stack_pop(mu, func);
                    mu.dynamic_pop();

                    Ok(value)
                }
                _ => Err(Exception::new(Condition::Type, "apply", func)),
            },
            _ => Err(Exception::new(Condition::Type, "apply", func)),
        }
    }
}

pub trait MuFunction {
    fn libcore_fr_pop(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_fr_push(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn libcore_fr_ref(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Frame {
    fn libcore_fr_pop(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.fp_argv_check("fr-pop", &[Type::Function], fp) {
            Ok(_) => {
                Self::frame_stack_pop(mu, fp.argv[0]);
                fp.argv[0]
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn libcore_fr_push(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.fp_argv_check("fr-push", &[Type::Vector], fp) {
            Ok(_) => {
                Self::from_tag(mu, fp.value).frame_stack_push(mu);
                fp.argv[0]
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn libcore_fr_ref(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let frame = fp.argv[0];
        let offset = fp.argv[1];

        fp.value = match mu.fp_argv_check("fr-ref", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Frame::frame_ref(
                mu,
                Fixnum::as_i64(frame) as u64,
                Fixnum::as_i64(offset) as usize,
            ) {
                Some(tag) => tag,
                None => return Err(Exception::new(Condition::Type, "fr-ref", frame)),
            },
            Err(e) => return Err(e),
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
