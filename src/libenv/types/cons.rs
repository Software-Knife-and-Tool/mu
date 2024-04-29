//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cons class
use crate::{
    core::{
        apply::Core as _,
        direct::{DirectInfo, DirectTag, DirectType},
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        gc::Core as _,
        indirect::IndirectTag,
        lib::LIB,
        types::{Tag, TagType, Type},
    },
    streams::{read::Core as _, write::Core as _},
    types::{
        fixnum::Fixnum,
        symbol::Symbol,
        vector::{TypedVec, VecType},
        vectors::Core as _,
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
        match tag.type_of() {
            Type::Cons => match tag {
                Tag::Indirect(main) => {
                    let heap_ref = block_on(env.heap.read());

                    Cons {
                        car: Tag::from_slice(
                            heap_ref.image_slice(main.image_id() as usize).unwrap(),
                        ),
                        cdr: Tag::from_slice(
                            heap_ref.image_slice(main.image_id() as usize + 1).unwrap(),
                        ),
                    }
                }
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
            _ => panic!("cons::length"),
        }
    }
}

// core operations
pub trait Core {
    fn evict(&self, _: &Env) -> Tag;
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn mark(_: &Env, _: Tag);
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

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn mark(env: &Env, cons: Tag) {
        match cons {
            Tag::Direct(_) => {
                env.mark(Self::car(env, cons));
                env.mark(Self::cdr(env, cons))
            }
            Tag::Indirect(_) => {
                let mark = env.mark_image(cons).unwrap();

                if !mark {
                    env.mark(Self::car(env, cons));
                    env.mark(Self::cdr(env, cons))
                }
            }
        }
    }

    fn iter(env: &Env, cons: Tag) -> ConsIter {
        ConsIter { env, cons }
    }

    fn heap_size(_: &Env, cons: Tag) -> usize {
        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info() {
                    DirectTag::EXT_TYPE_CONS => std::mem::size_of::<DirectTag>(),
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

        match env.read_stream(stream, false, Tag::nil(), true) {
            Ok(car) => {
                if LIB.eol.eq_(&car) {
                    Ok(Tag::nil())
                } else {
                    match car.type_of() {
                        Type::Symbol if dot.eq_(&Symbol::name(env, car)) => {
                            match env.read_stream(stream, false, Tag::nil(), true) {
                                Ok(cdr) if LIB.eol.eq_(&cdr) => Ok(Tag::nil()),
                                Ok(cdr) => match env.read_stream(stream, false, Tag::nil(), true) {
                                    Ok(eol) if LIB.eol.eq_(&eol) => Ok(cdr),
                                    Ok(_) => Err(Exception::new(Condition::Eof, "car", stream)),
                                    Err(e) => Err(e),
                                },
                                Err(e) => Err(e),
                            }
                        }
                        _ => match Self::read(env, stream) {
                            Ok(cdr) => Ok(Cons::new(car, cdr).evict(env)),
                            Err(e) => Err(e),
                        },
                    }
                }
            }
            Err(e) => Err(e),
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
pub trait LibFunction {
    fn lib_append(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_car(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_cdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_cons(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_nth(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_nthcdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Cons {
    fn lib_append(env: &Env, fp: &mut Frame) -> exception::Result<()> {
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

    fn lib_car(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::car(env, list),
            _ => return Err(Exception::new(Condition::Type, "car", list)),
        };

        Ok(())
    }

    fn lib_cdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::cdr(env, list),
            _ => return Err(Exception::new(Condition::Type, "cdr", list)),
        };

        Ok(())
    }

    fn lib_cons(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::new(fp.argv[0], fp.argv[1]).evict(env);
        Ok(())
    }

    fn lib_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => Tag::from(0i64),
            Type::Cons => match Cons::length(env, list) {
                Some(len) => Tag::from(len as i64),
                None => return Err(Exception::new(Condition::Type, "length", list)),
            },
            _ => return Err(Exception::new(Condition::Type, "length", list)),
        };

        Ok(())
    }

    fn lib_nth(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let nth = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match env.fp_argv_check("nth", &[Type::Fixnum, Type::List], fp) {
            Ok(_) => {
                if Fixnum::as_i64(nth) < 0 {
                    return Err(Exception::new(Condition::Type, "nth", nth));
                }

                match list.type_of() {
                    Type::Null => Tag::nil(),
                    Type::Cons => match Self::nth(env, Fixnum::as_i64(nth) as usize, list) {
                        Some(tag) => tag,
                        None => return Err(Exception::new(Condition::Type, "nth", list)),
                    },
                    _ => panic!(),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_nthcdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let nth = fp.argv[0];
        let list = fp.argv[1];

        fp.value = match env.fp_argv_check("nthcdr", &[Type::Fixnum, Type::List], fp) {
            Ok(_) => {
                if Fixnum::as_i64(nth) < 0 {
                    return Err(Exception::new(Condition::Type, "nth", nth));
                }

                match list.type_of() {
                    Type::Null => Tag::nil(),
                    Type::Cons => match Self::nthcdr(env, Fixnum::as_i64(nth) as usize, list) {
                        Some(tag) => tag,
                        None => return Err(Exception::new(Condition::Type, "nthcdr", list)),
                    },
                    _ => panic!(),
                }
            }
            Err(e) => return Err(e),
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
