//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cons class
use crate::{
    core::{
        apply::Core as _,
        direct::{DirectInfo, DirectTag, DirectType, ExtType},
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::{Gc, HeapGcRef},
        indirect::IndirectTag,
        reader::EOL,
        types::{Tag, TagType, Type},
    },
    streams::{read::Core as _, write::Core as _},
    types::{
        fixnum::{Core as _, Fixnum},
        indirect_vector::Core as _,
        symbol::Symbol,
        vector::Vector,
    },
};

use futures::executor::block_on;

#[derive(Copy, Clone)]
pub struct Cons {
    car: Tag,
    cdr: Tag,
}

impl Cons {
    pub fn new(car: Tag, cdr: Tag) -> Self {
        Cons { car, cdr }
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Cons => match tag {
                Tag::Indirect(main) => Cons {
                    car: Tag::from_slice(heap_ref.image_slice(main.image_id() as usize).unwrap()),
                    cdr: Tag::from_slice(
                        heap_ref.image_slice(main.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> Self {
        match tag.type_of() {
            Type::Cons => match tag {
                Tag::Indirect(main) => Cons {
                    car: Tag::from_slice(heap_ref.image_slice(main.image_id() as usize).unwrap()),
                    cdr: Tag::from_slice(
                        heap_ref.image_slice(main.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn car(env: &Env, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::car(cons),
                Tag::Indirect(_) => Self::to_image(env, cons).car,
            },
            _ => panic!(),
        }
    }

    pub fn cdr(env: &Env, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::to_image(env, cons).cdr,
                Tag::Direct(_) => DirectTag::cdr(cons),
            },
            _ => panic!(),
        }
    }

    pub fn ref_car(gc: &mut Gc, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::car(cons),
                Tag::Indirect(_) => Self::gc_ref_image(&mut gc.lock, cons).car,
            },
            _ => panic!(),
        }
    }

    pub fn ref_cdr(gc: &mut Gc, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::gc_ref_image(&mut gc.lock, cons).cdr,
                Tag::Direct(_) => DirectTag::cdr(cons),
            },
            _ => panic!(),
        }
    }

    pub fn length(env: &Env, cons: Tag) -> Option<usize> {
        match cons.type_of() {
            Type::Null => Some(0),
            Type::Cons => {
                let mut cp = cons;
                let mut n = 0;

                loop {
                    match cp.type_of() {
                        Type::Cons => {
                            n += 1;
                            cp = Self::cdr(env, cp)
                        }
                        Type::Null => break,
                        _ => return None,
                    }
                }

                Some(n)
            }
            _ => panic!(),
        }
    }

    pub fn mark(gc: &mut Gc, env: &Env, cons: Tag) {
        match cons {
            Tag::Direct(_) => {
                let car = Self::ref_car(gc, cons);
                let cdr = Self::ref_cdr(gc, cons);

                gc.mark(env, car);
                gc.mark(env, cdr)
            }
            Tag::Indirect(_) => {
                let mark = gc.mark_image(cons).unwrap();
                if !mark {
                    let car = Self::ref_car(gc, cons);
                    let cdr = Self::ref_cdr(gc, cons);

                    gc.mark(env, car);
                    gc.mark(env, cdr)
                }
            }
        }
    }
}

// core operations
pub trait Core {
    fn evict(&self, _: &Env) -> Tag;
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn iter(_: &Env, _: Tag) -> ConsIter;
    fn nth(_: &Env, _: usize, _: Tag) -> Option<Tag>;
    fn nthcdr(_: &Env, _: usize, _: Tag) -> Option<Tag>;
    fn read(_: &Env, _: Tag) -> exception::Result<Tag>;
    fn vappend(_: &Env, _: &[Tag], _: Tag) -> Tag;
    fn view(_: &Env, _: Tag) -> Tag;
    fn vlist(_: &Env, _: &[Tag]) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Cons {
    fn view(env: &Env, cons: Tag) -> Tag {
        let vec = vec![Self::car(env, cons), Self::cdr(env, cons)];

        Vector::from(vec).evict(env)
    }

    fn iter(env: &Env, cons: Tag) -> ConsIter {
        ConsIter { env, cons }
    }

    fn heap_size(_: &Env, cons: Tag) -> usize {
        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info().try_into() {
                    Ok(ExtType::Cons) => std::mem::size_of::<DirectTag>(),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Tag::Indirect(_) => std::mem::size_of::<Cons>(),
        }
    }

    fn evict(&self, env: &Env) -> Tag {
        match DirectTag::cons(self.car, self.cdr) {
            Some(tag) => tag,
            None => {
                let image: &[[u8; 8]] = &[self.car.as_slice(), self.cdr.as_slice()];
                let mut heap_ref = block_on(env.heap.write());

                let ind = IndirectTag::new()
                    .with_image_id(heap_ref.alloc(image, None, Type::Cons as u8).unwrap() as u64)
                    .with_heap_id(1)
                    .with_tag(TagType::Cons);

                Tag::Indirect(ind)
            }
        }
    }

    fn read(env: &Env, stream: Tag) -> exception::Result<Tag> {
        let dot = DirectTag::to_direct('.' as u64, DirectInfo::Length(1), DirectType::ByteVector);

        let car = env.read_stream(stream, false, Tag::nil(), true)?;

        if EOL.eq_(&car) {
            Ok(Tag::nil())
        } else {
            match car.type_of() {
                Type::Symbol if dot.eq_(&Symbol::name(env, car)) => {
                    let cdr = env.read_stream(stream, false, Tag::nil(), true)?;

                    if EOL.eq_(&cdr) {
                        Ok(Tag::nil())
                    } else {
                        let eol = env.read_stream(stream, false, Tag::nil(), true)?;

                        if EOL.eq_(&eol) {
                            Ok(cdr)
                        } else {
                            Err(Exception::new(env, Condition::Eof, "mu:read", stream))
                        }
                    }
                }
                _ => {
                    let cdr = Self::read(env, stream)?;

                    Ok(Cons::new(car, cdr).evict(env))
                }
            }
        }
    }

    fn write(env: &Env, cons: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let car = Self::car(env, cons);

        env.write_string("(", stream).unwrap();
        env.write_stream(car, escape, stream).unwrap();

        let mut tail = Self::cdr(env, cons);

        // this is ugly, but it might be worse with a for loop
        loop {
            match tail.type_of() {
                Type::Cons => {
                    env.write_string(" ", stream).unwrap();
                    env.write_stream(Self::car(env, tail), escape, stream)
                        .unwrap();
                    tail = Self::cdr(env, tail);
                }
                _ if tail.null_() => break,
                _ => {
                    env.write_string(" . ", stream).unwrap();
                    env.write_stream(tail, escape, stream).unwrap();
                    break;
                }
            }
        }

        env.write_string(")", stream)
    }

    fn vlist(env: &Env, vec: &[Tag]) -> Tag {
        let mut list = Tag::nil();

        vec.iter()
            .rev()
            .for_each(|tag| list = Self::new(*tag, list).evict(env));

        list
    }

    fn vappend(env: &Env, vec: &[Tag], cdr: Tag) -> Tag {
        let mut list = cdr;

        vec.iter()
            .rev()
            .for_each(|tag| list = Self::new(*tag, list).evict(env));

        list
    }

    fn nth(env: &Env, n: usize, cons: Tag) -> Option<Tag> {
        let mut nth = n;
        let mut tail = cons;

        match cons.type_of() {
            Type::Null => Some(Tag::nil()),
            Type::Cons => loop {
                match tail.type_of() {
                    _ if tail.null_() => return Some(Tag::nil()),
                    Type::Cons => {
                        if nth == 0 {
                            return Some(Self::car(env, tail));
                        }
                        nth -= 1;
                        tail = Self::cdr(env, tail)
                    }
                    _ => {
                        return if nth != 0 { None } else { Some(tail) };
                    }
                }
            },
            _ => panic!(),
        }
    }

    fn nthcdr(env: &Env, n: usize, cons: Tag) -> Option<Tag> {
        let mut nth = n;
        let mut tail = cons;

        match cons.type_of() {
            Type::Null => Some(Tag::nil()),
            Type::Cons => loop {
                match tail.type_of() {
                    _ if tail.null_() => return Some(Tag::nil()),
                    Type::Cons => {
                        if nth == 0 {
                            return Some(tail);
                        }
                        nth -= 1;
                        tail = Self::cdr(env, tail)
                    }
                    _ => {
                        return if nth != 0 { None } else { Some(tail) };
                    }
                }
            },
            _ => panic!(),
        }
    }
}

/// env functions
pub trait CoreFunction {
    fn mu_append(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_car(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_cdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_cons(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_nth(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_nthcdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Cons {
    fn mu_append(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list1 = fp.argv[0];
        let list2 = fp.argv[1];

        fp.value = match list1.type_of() {
            Type::Null | Type::Cons => Cons::vappend(
                env,
                &Cons::iter(env, list1)
                    .map(|elt| Cons::car(env, elt))
                    .collect::<Vec<Tag>>(),
                list2,
            ),
            _ => list1,
        };

        Ok(())
    }

    fn mu_car(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        env.fp_argv_check("mu:car", &[Type::List], fp)?;
        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::car(env, list),
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_cdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        env.fp_argv_check("mu:cdr", &[Type::List], fp)?;
        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::cdr(env, list),
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_cons(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::new(fp.argv[0], fp.argv[1]).evict(env);

        Ok(())
    }

    fn mu_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        env.fp_argv_check("mu:length", &[Type::List], fp)?;
        fp.value = match list.type_of() {
            Type::Null => Fixnum::with_or_panic(0),
            Type::Cons => match Cons::length(env, list) {
                Some(len) => Fixnum::with_or_panic(len),
                None => return Err(Exception::new(env, Condition::Type, "mu:length", list)),
            },
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_nth(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let nth = fp.argv[0];
        let list = fp.argv[1];

        env.fp_argv_check("mu:nth", &[Type::Fixnum, Type::List], fp)?;

        if Fixnum::as_i64(nth) < 0 {
            return Err(Exception::new(env, Condition::Type, "mu:nth", nth));
        }

        fp.value = match list.type_of() {
            Type::Null => Tag::nil(),
            Type::Cons => match Self::nth(env, Fixnum::as_i64(nth) as usize, list) {
                Some(tag) => tag,
                None => return Err(Exception::new(env, Condition::Type, "mu:nth", list)),
            },
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_nthcdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let nth = fp.argv[0];
        let list = fp.argv[1];

        env.fp_argv_check("mu:nthcdr", &[Type::Fixnum, Type::List], fp)?;

        if Fixnum::as_i64(nth) < 0 {
            return Err(Exception::new(env, Condition::Type, "mu:nthcdr", nth));
        }

        fp.value = match list.type_of() {
            Type::Null => Tag::nil(),
            Type::Cons => match Self::nthcdr(env, Fixnum::as_i64(nth) as usize, list) {
                Some(tag) => tag,
                None => return Err(Exception::new(env, Condition::Type, "mu:nthcdr", list)),
            },
            _ => panic!(),
        };

        Ok(())
    }
}

// iterator
pub struct ConsIter<'a> {
    env: &'a Env,
    pub cons: Tag,
}

// proper lists only
impl<'a> Iterator for ConsIter<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cons.type_of() {
            Type::Cons => {
                let cons = self.cons;
                self.cons = Cons::cdr(self.env, self.cons);
                Some(cons)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::Tag;
    use crate::types::cons::Cons;

    #[test]
    fn cons() {
        match Cons::new(Tag::nil(), Tag::nil()) {
            _ => assert_eq!(true, true),
        }
    }
}
