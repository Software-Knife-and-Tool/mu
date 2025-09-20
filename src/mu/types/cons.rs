//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cons type
use {
    crate::{
        core_::{
            apply::Apply as _,
            direct::{DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            indirect::IndirectTag,
            reader::{Reader, EOL},
            tag::{Tag, TagType},
            type_::Type,
            writer::Writer,
        },
        spaces::{
            gc::{Gc as _, GcContext},
            heap::HeapRequest,
        },
        streams::writer::StreamWriter,
        types::{fixnum::Fixnum, symbol::Symbol, vector::Vector},
    },
    futures_lite::future::block_on,
};

#[derive(Copy, Clone)]
pub struct Cons {
    car: Tag,
    cdr: Tag,
}

pub trait Gc {
    fn gc_ref_image(_: &GcContext, tag: Tag) -> Self;
    fn ref_car(_: &GcContext, _: Tag) -> Tag;
    fn ref_cdr(_: &GcContext, _: Tag) -> Tag;
    fn mark(_: &mut GcContext, _: &Env, _: Tag);
}

impl Gc for Cons {
    fn gc_ref_image(context: &GcContext, tag: Tag) -> Self {
        let heap_ref = &context.heap_ref;

        assert_eq!(tag.type_of(), Type::Cons);
        match tag {
            Tag::Indirect(main) => Cons {
                car: Tag::from_slice(heap_ref.image_slice(main.image_id() as usize).unwrap()),
                cdr: Tag::from_slice(heap_ref.image_slice(main.image_id() as usize + 1).unwrap()),
            },
            _ => panic!(),
        }
    }

    fn ref_car(context: &GcContext, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::cons_destruct(cons).0,
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).car,
            },
            _ => panic!(),
        }
    }

    fn ref_cdr(context: &GcContext, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).cdr,
                Tag::Direct(_) => DirectTag::cons_destruct(cons).1,
            },
            _ => panic!(),
        }
    }

    fn mark(context: &mut GcContext, env: &Env, cons: Tag) {
        match cons {
            Tag::Direct(_) => {
                let car = Self::ref_car(context, cons);
                let cdr = Self::ref_cdr(context, cons);

                context.mark(env, car);
                context.mark(env, cdr)
            }
            Tag::Indirect(_) => {
                let mark = context.mark_image(cons).unwrap();
                if !mark {
                    context.mark(env, Self::ref_car(context, cons));
                    context.mark(env, Self::ref_cdr(context, cons));
                }
            }
        }
    }
}

impl Cons {
    pub fn new(car: Tag, cdr: Tag) -> Self {
        Cons { car, cdr }
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        assert_eq!(tag.type_of(), Type::Cons);

        let heap_ref = block_on(env.heap.read());

        match tag {
            Tag::Indirect(cons) => Self::new(
                Tag::from_slice(heap_ref.image_slice(cons.image_id() as usize).unwrap()),
                Tag::from_slice(heap_ref.image_slice(cons.image_id() as usize + 1).unwrap()),
            ),
            Tag::Direct(_) => panic!(),
        }
    }

    pub fn destruct(env: &Env, cons: Tag) -> (Tag, Tag) {
        assert!(cons.type_of() == Type::Cons || cons.type_of() == Type::Null);

        match cons {
            Tag::Indirect(cons) => {
                let heap_ref = block_on(env.heap.read());

                (
                    Tag::from_slice(heap_ref.image_slice(cons.image_id() as usize).unwrap()),
                    Tag::from_slice(heap_ref.image_slice(cons.image_id() as usize + 1).unwrap()),
                )
            }
            Tag::Direct(_) => DirectTag::cons_destruct(cons),
        }
    }

    pub fn cons(env: &Env, car: Tag, cdr: Tag) -> Tag {
        Self::new(car, cdr).evict(env)
    }

    pub fn list(env: &Env, vec: &[Tag]) -> Tag {
        let mut list = Tag::nil();

        vec.iter()
            .rev()
            .for_each(|tag| list = Self::cons(env, *tag, list));

        list
    }

    pub fn cons_deref(env: &Env, cons: Tag) -> (Tag, Tag) {
        let car = match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::cons_destruct(cons).0,
                Tag::Indirect(_) => Self::to_image(env, cons).car,
            },
            _ => panic!(),
        };

        let cdr = match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::cons_destruct(cons).1,
                Tag::Indirect(_) => Self::to_image(env, cons).cdr,
            },
            _ => panic!(),
        };

        (car, cdr)
    }

    /*
        pub fn car(env: &Env, cons: Tag) -> Tag {
            match cons.type_of() {
                Type::Null => cons,
                Type::Cons => match cons {
                    Tag::Direct(_) => DirectTag::cons_deref(cons).0,
                    Tag::Image(_) | Tag::Indirect(_) => Self::to_image(env, cons).car,
                },
                _ => panic!(),
            }
        }

        pub fn cdr(env: &Env, cons: Tag) -> Tag {
            match cons.type_of() {
                Type::Null => cons,
                Type::Cons => match cons {
                    Tag::Direct(_) => DirectTag::cons_deref(cons).1,
                    Tag::Image(_) | Tag::Indirect(_) => Self::to_image(env, cons).cdr,
                },
                _ => panic!(),
            }
    }
        */

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
                            cp = Self::destruct(env, cp).1
                        }
                        Type::Null => break,
                        _ => None?,
                    }
                }

                Some(n)
            }
            _ => panic!(),
        }
    }

    pub fn view(env: &Env, cons: Tag) -> Tag {
        let (car, cdr) = Self::destruct(env, cons);

        Vector::from(vec![car, cdr]).evict(env)
    }

    pub fn cons_iter(env: &Env, cons: Tag) -> ConsIter<'_> {
        ConsIter { env, cons }
    }

    pub fn list_iter(env: &Env, list: Tag) -> ListIter<'_> {
        ListIter { env, list }
    }

    pub fn heap_size(_: &Env, cons: Tag) -> usize {
        match cons {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.ext().try_into() {
                    Ok(ExtType::Cons) => std::mem::size_of::<DirectTag>(),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            Tag::Indirect(_) => std::mem::size_of::<Cons>(),
        }
    }

    pub fn evict(&self, env: &Env) -> Tag {
        match DirectTag::cons(self.car, self.cdr) {
            Some(tag) => tag,
            None => {
                let image: &[[u8; 8]] = &[self.car.as_slice(), self.cdr.as_slice()];
                let ha = HeapRequest {
                    env,
                    image,
                    vdata: None,
                    type_id: Type::Cons as u8,
                };
                let heap_ref = &mut block_on(env.heap.write());

                match heap_ref.alloc(&ha) {
                    Some(image_id) => {
                        let ind = IndirectTag::new()
                            .with_image_id(image_id as u64)
                            .with_heap_id(1)
                            .with_tag(TagType::Cons);

                        Tag::Indirect(ind)
                    }
                    None => {
                        panic!()
                    }
                }
            }
        }
    }

    pub fn read(env: &Env, stream: Tag) -> exception::Result<Tag> {
        let dot = Vector::from(".").evict(env);
        let car = env.read(stream, false, Tag::nil(), true)?;

        if EOL.eq_(&car) {
            return Ok(Tag::nil());
        }

        match car.type_of() {
            Type::Symbol if dot.eq_(&Symbol::destruct(env, car).1) => {
                let cdr = env.read(stream, false, Tag::nil(), true)?;

                if EOL.eq_(&cdr) {
                    Ok(Tag::nil())
                } else {
                    let eol = env.read(stream, false, Tag::nil(), true)?;

                    if EOL.eq_(&eol) {
                        Ok(cdr)
                    } else {
                        Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                    }
                }
            }
            _ => Ok(Cons::cons(env, car, Self::read(env, stream)?)),
        }
    }

    pub fn write(env: &Env, cons: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let (car, mut tail) = Self::destruct(env, cons);

        StreamWriter::write_char(env, stream, '(').unwrap();
        env.write(car, escape, stream).unwrap();

        // this is ugly, but it might be worse with a for loop
        loop {
            match tail.type_of() {
                Type::Cons => {
                    let (car, cdr) = Self::destruct(env, tail);
                    StreamWriter::write_char(env, stream, ' ').unwrap();
                    env.write(car, escape, stream).unwrap();
                    tail = cdr;
                }
                _ if tail.null_() => break,
                _ => {
                    StreamWriter::write_str(env, " . ", stream).unwrap();
                    env.write(tail, escape, stream).unwrap();
                    break;
                }
            }
        }

        StreamWriter::write_str(env, ")", stream)
    }

    pub fn append(env: &Env, vec: &[Tag], cdr: Tag) -> Tag {
        let mut list = cdr;

        vec.iter()
            .rev()
            .for_each(|tag| list = Self::cons(env, *tag, list));

        list
    }

    pub fn nth(env: &Env, n: usize, cons: Tag) -> Option<Tag> {
        let mut nth = n;
        let mut tail = cons;

        match cons.type_of() {
            Type::Null => Some(Tag::nil()),
            Type::Cons => loop {
                match tail.type_of() {
                    _ if tail.null_() => return Some(Tag::nil()),
                    Type::Cons => {
                        let (car, cdr) = Self::destruct(env, tail);
                        if nth == 0 {
                            return Some(car);
                        }
                        nth -= 1;
                        tail = cdr
                    }
                    _ => {
                        return if nth != 0 { None } else { Some(tail) };
                    }
                }
            },
            _ => panic!(),
        }
    }

    pub fn nthcdr(env: &Env, n: usize, cons: Tag) -> Option<Tag> {
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
                        tail = Self::destruct(env, tail).1
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
pub trait CoreFn {
    fn mu_append(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_car(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_cdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_cons(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_nth(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_nthcdr(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Cons {
    fn mu_append(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:car", &[Type::List], fp)?;

        let lists = fp.argv[0];

        fp.value = Tag::nil();
        if !lists.null_() {
            let mut appended = vec![];

            for cons in Cons::cons_iter(env, lists) {
                let (list, cdr) = Cons::destruct(env, cons);

                if cdr.null_() {
                    fp.value = Self::append(env, &appended, list)
                } else {
                    match list.type_of() {
                        Type::Null => (),
                        Type::Cons => {
                            for car in Cons::list_iter(env, list) {
                                appended.push(car)
                            }
                        }
                        _ => Err(Exception::new(env, Condition::Type, "mu:append", list))?,
                    }
                }
            }
        }

        Ok(())
    }

    fn mu_car(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:car", &[Type::List], fp)?;

        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::destruct(env, list).0,
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_cdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:cdr", &[Type::List], fp)?;

        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::destruct(env, list).1,
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_cons(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Self::cons(env, fp.argv[0], fp.argv[1]);

        Ok(())
    }

    fn mu_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:length", &[Type::List], fp)?;

        let list = fp.argv[0];

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
        env.argv_check("mu:nth", &[Type::Fixnum, Type::List], fp)?;

        let nth = fp.argv[0];
        let list = fp.argv[1];

        if Fixnum::as_i64(nth) < 0 {
            Err(Exception::new(env, Condition::Type, "mu:nth", nth))?
        }

        fp.value = match list.type_of() {
            Type::Null => Tag::nil(),
            Type::Cons => match Self::nth(env, Fixnum::as_i64(nth) as usize, list) {
                Some(tag) => tag,
                None => Err(Exception::new(env, Condition::Type, "mu:nth", list))?,
            },
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_nthcdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:nthcdr", &[Type::Fixnum, Type::List], fp)?;

        let nth = fp.argv[0];
        let list = fp.argv[1];

        if Fixnum::as_i64(nth) < 0 {
            return Err(Exception::new(env, Condition::Type, "mu:nthcdr", nth))?;
        }

        fp.value = match list.type_of() {
            Type::Null => Tag::nil(),
            Type::Cons => match Self::nthcdr(env, Fixnum::as_i64(nth) as usize, list) {
                Some(tag) => tag,
                None => Err(Exception::new(env, Condition::Type, "mu:nthcdr", list))?,
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
impl Iterator for ConsIter<'_> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cons.type_of() {
            Type::Cons => {
                let cons = self.cons;

                self.cons = Cons::destruct(self.env, self.cons).1;
                Some(cons)
            }
            _ => None,
        }
    }
}

// iterator
pub struct ListIter<'a> {
    env: &'a Env,
    pub list: Tag,
}

// proper lists only
impl Iterator for ListIter<'_> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.type_of() {
            Type::Cons => {
                let (car, cdr) = Cons::destruct(self.env, self.list);

                self.list = cdr;
                Some(car)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core_::tag::Tag, types::cons::Cons};

    #[test]
    fn cons_test() {
        match Cons::new(Tag::nil(), Tag::nil()) {
            _ => assert!(true),
        }
    }
}
