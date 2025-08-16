//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! cons type
use {
    crate::{
        core::{
            apply::Apply as _,
            direct::{DirectTag, DirectType, ExtType},
            dynamic::Dynamic,
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::{Gc as _, GcContext},
            heap::HeapRequest,
            indirect::IndirectTag,
            reader::{Reader, EOL},
            type_image::TypeImage,
            types::{Tag, TagType, Type},
            writer::Writer,
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
                Tag::Image(_) => panic!(),
                Tag::Direct(_) => DirectTag::car(cons),
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).car,
            },
            _ => panic!(),
        }
    }

    fn ref_cdr(context: &GcContext, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Image(_) => panic!(),
                Tag::Indirect(_) => Self::gc_ref_image(context, cons).cdr,
                Tag::Direct(_) => DirectTag::cdr(cons),
            },
            _ => panic!(),
        }
    }

    fn mark(context: &mut GcContext, env: &Env, cons: Tag) {
        match cons {
            Tag::Image(_) => panic!(),
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

    pub fn to_image_tag(self, env: &Env) -> Tag {
        let image = TypeImage::Cons(self);

        TypeImage::to_tag(&image, env, Type::Cons as u8)
    }

    pub fn to_image(env: &Env, tag: Tag) -> Self {
        let heap_ref = block_on(env.heap.read());

        assert_eq!(tag.type_of(), Type::Cons);

        match tag {
            Tag::Image(image) => match Dynamic::images_ref(env, image.data() as usize) {
                TypeImage::Cons(cons) => cons,
                _ => panic!(),
            },
            Tag::Indirect(main) => Self::new(
                Tag::from_slice(heap_ref.image_slice(main.image_id() as usize).unwrap()),
                Tag::from_slice(heap_ref.image_slice(main.image_id() as usize + 1).unwrap()),
            ),
            _ => panic!(),
        }
    }

    pub fn cons_image(env: &Env, car: Tag, cdr: Tag) -> Tag {
        Self::to_image_tag(Self::new(car, cdr), env)
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

    pub fn car(env: &Env, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::car(cons),
                Tag::Image(_) | Tag::Indirect(_) => Self::to_image(env, cons).car,
            },
            _ => panic!(),
        }
    }

    pub fn cdr(env: &Env, cons: Tag) -> Tag {
        match cons.type_of() {
            Type::Null => cons,
            Type::Cons => match cons {
                Tag::Direct(_) => DirectTag::cdr(cons),
                Tag::Image(_) | Tag::Indirect(_) => Self::to_image(env, cons).cdr,
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

    pub fn view(env: &Env, cons: Tag) -> Tag {
        let vec = vec![Self::car(env, cons), Self::cdr(env, cons)];

        Vector::from(vec).evict(env)
    }

    pub fn iter(env: &Env, cons: Tag) -> ConsIter<'_> {
        ConsIter { env, cons }
    }

    pub fn heap_size(_: &Env, cons: Tag) -> usize {
        match cons {
            Tag::Image(_) => panic!(),
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

    pub fn evict_image(tag: Tag, env: &Env) -> Tag {
        match tag {
            Tag::Image(_) => Self::to_image(env, tag).evict(env),
            _ => panic!(),
        }
    }

    pub fn read(env: &Env, stream: Tag) -> exception::Result<Tag> {
        let dot = Vector::from(".").evict(env);
        let car = env.read(stream, false, Tag::nil(), true)?;

        if EOL.eq_(&car) {
            return Ok(Tag::nil());
        }

        match car.type_of() {
            Type::Symbol if dot.eq_(&Symbol::name(env, car)) => {
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

    pub fn read_image(env: &Env, stream: Tag) -> exception::Result<Tag> {
        let dot = Vector::from(".").evict(env);
        let car = env.read(stream, false, Tag::nil(), true)?;

        if EOL.eq_(&car) {
            return Ok(Tag::nil());
        }

        match car.type_of() {
            Type::Symbol if dot.eq_(&Symbol::name(env, car)) => {
                let cdr = env.read(stream, false, Tag::nil(), true)?;

                if EOL.eq_(&cdr) {
                    Ok(Tag::nil())
                } else if EOL.eq_(&env.read(stream, false, Tag::nil(), true)?) {
                    Ok(cdr)
                } else {
                    Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                }
            }
            _ => Ok(Cons::cons_image(env, car, Self::read(env, stream)?)),
        }
    }

    pub fn write(env: &Env, cons: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        let car = Self::car(env, cons);

        StreamWriter::write_char(env, stream, '(').unwrap();
        env.write(car, escape, stream).unwrap();

        let mut tail = Self::cdr(env, cons);

        // this is ugly, but it might be worse with a for loop
        loop {
            match tail.type_of() {
                Type::Cons => {
                    StreamWriter::write_char(env, stream, ' ').unwrap();
                    env.write(Self::car(env, tail), escape, stream).unwrap();
                    tail = Self::cdr(env, tail);
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
        env.argv_check("mu:car", &[Type::List], fp)?;

        let lists = fp.argv[0];

        fp.value = Tag::nil();
        if !lists.null_() {
            let mut appended = vec![];

            for cons in Cons::iter(env, lists) {
                let list = Self::car(env, cons);

                if Self::cdr(env, cons).null_() {
                    fp.value = Self::append(env, &appended, list)
                } else {
                    match list.type_of() {
                        Type::Null => (),
                        Type::Cons => {
                            for cons in Cons::iter(env, list) {
                                appended.push(Self::car(env, cons))
                            }
                        }
                        _ => return Err(Exception::new(env, Condition::Type, "mu:append", list))?,
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
            Type::Cons => Self::car(env, list),
            _ => panic!(),
        };

        Ok(())
    }

    fn mu_cdr(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:cdr", &[Type::List], fp)?;

        let list = fp.argv[0];

        fp.value = match list.type_of() {
            Type::Null => list,
            Type::Cons => Self::cdr(env, list),
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
                self.cons = Cons::cdr(self.env, self.cons);
                Some(cons)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::types::Tag, types::cons::Cons};

    #[test]
    fn cons() {
        match Cons::new(Tag::nil(), Tag::nil()) {
            _ => assert!(true),
        }
    }
}
