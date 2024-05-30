//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env vector type
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            gc::{Gc, HeapGcRef},
            readtable::{map_char_syntax, SyntaxType},
            types::{Tag, Type},
        },
        streams::write::Core as _,
        types::{
            char::Char,
            cons::{Cons, Core as _},
            core_stream::{Core as _, Stream},
            fixnum::{Core as _, Fixnum},
            float::Float,
            indirect_vector::VectorImage,
            indirect_vector::{IVec, IVector, IndirectVector},
            indirect_vector::{TypedVector, VecType, VectorIter},
            symbol::{Core as _, Symbol},
        },
    },
    futures::executor::block_on,
    futures_locks::RwLock,
    std::{collections::HashMap, str},
};

pub type VecCacheMap = HashMap<(Type, i32), RwLock<Vec<Tag>>>;

pub enum Vector {
    Direct(Tag),
    Indirect(VectorImage, IVec),
}

lazy_static! {
    static ref VTYPEMAP: Vec<(Tag, Type)> = vec![
        (Symbol::keyword("t"), Type::T),
        (Symbol::keyword("char"), Type::Char),
        (Symbol::keyword("byte"), Type::Byte),
        (Symbol::keyword("fixnum"), Type::Fixnum),
        (Symbol::keyword("float"), Type::Float),
    ];
}

impl Vector {
    const IMAGE_LEN: usize = 2; // heap words in image

    pub fn to_type(keyword: Tag) -> Option<Type> {
        VTYPEMAP
            .iter()
            .copied()
            .find(|tab| keyword.eq_(&tab.0))
            .map(|tab| tab.1)
    }

    pub fn to_image(env: &Env, tag: Tag) -> VectorImage {
        let heap_ref = block_on(env.heap.read());

        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => VectorImage {
                    vtype: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    length: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn gc_ref_image(heap_ref: &mut HeapGcRef, tag: Tag) -> VectorImage {
        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Indirect(image) => VectorImage {
                    vtype: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize).unwrap(),
                    ),
                    length: Tag::from_slice(
                        heap_ref.image_slice(image.image_id() as usize + 1).unwrap(),
                    ),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn type_of(env: &Env, vector: Tag) -> Type {
        match vector {
            Tag::Direct(_) => Type::Char,
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);

                match VTYPEMAP
                    .iter()
                    .copied()
                    .find(|desc| image.vtype.eq_(&desc.0))
                {
                    Some(desc) => desc.1,
                    None => panic!(),
                }
            }
        }
    }

    pub fn length(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Direct(direct) => direct.info() as usize,
            Tag::Indirect(_) => {
                let image = Self::to_image(env, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    pub fn ref_type_of(gc: &mut Gc, vector: Tag) -> Type {
        match vector {
            Tag::Direct(_) => Type::Char,
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(&mut gc.lock, vector);

                match VTYPEMAP
                    .iter()
                    .copied()
                    .find(|desc| image.vtype.eq_(&desc.0))
                {
                    Some(desc) => desc.1,
                    None => panic!(),
                }
            }
        }
    }

    pub fn ref_length(gc: &mut Gc, vector: Tag) -> usize {
        match vector {
            Tag::Direct(direct) => direct.info() as usize,
            Tag::Indirect(_) => {
                let image = Self::gc_ref_image(&mut gc.lock, vector);
                Fixnum::as_i64(image.length) as usize
            }
        }
    }

    pub fn cache(env: &Env, vector: Tag) {
        let vtype = Self::type_of(env, vector);
        let length = Self::length(env, vector) as i32;
        let mut cache = block_on(env.vector_map.write());

        match (*cache).get(&(vtype, length)) {
            Some(vec_map) => {
                let mut vec = block_on(vec_map.write());

                vec.push(vector)
            }
            None => {
                if (*cache)
                    .insert((vtype, length), RwLock::new(vec![vector]))
                    .is_some()
                {
                    panic!()
                }
            }
        }
    }

    pub fn cached(env: &Env, indirect: &IndirectVector) -> Option<Tag> {
        let cache = block_on(env.vector_map.read());

        let (vtype, length, ivec) = match indirect {
            IndirectVector::Byte(image, ivec)
            | IndirectVector::Char(image, ivec)
            | IndirectVector::Fixnum(image, ivec)
            | IndirectVector::Float(image, ivec) => {
                (image.vtype, Fixnum::as_i64(image.length) as i32, ivec)
            }
            _ => panic!(),
        };

        match (*cache).get(&(Tag::key_type(vtype).unwrap(), length)) {
            Some(vec_map) => {
                let tag_vec = block_on(vec_map.read());

                let tag = match ivec {
                    IVec::Byte(u8_vec) => tag_vec.iter().find(|src| {
                        u8_vec.iter().enumerate().all(|(index, byte)| {
                            *byte as i64
                                == Fixnum::as_i64(Vector::ref_heap(env, **src, index).unwrap())
                        })
                    }),
                    IVec::Char(string) => tag_vec
                        .iter()
                        .find(|src| *string == Vector::as_string(env, **src)),
                    IVec::Fixnum(i64_vec) => tag_vec.iter().find(|src| {
                        i64_vec.iter().enumerate().all(|(index, fixnum)| {
                            *fixnum == Fixnum::as_i64(Vector::ref_heap(env, **src, index).unwrap())
                        })
                    }),
                    IVec::Float(float_vec) => tag_vec.iter().find(|src| {
                        float_vec.iter().enumerate().all(|(index, float)| {
                            *float
                                == Float::as_f32(env, Vector::ref_heap(env, **src, index).unwrap())
                        })
                    }),
                    _ => panic!(),
                };

                tag.copied()
            }
            None => None,
        }
    }

    pub fn mark(gc: &mut Gc, env: &Env, vector: Tag) {
        match vector {
            Tag::Direct(_) => (),
            Tag::Indirect(_) => {
                let marked = gc.mark_image(vector).unwrap();

                if !marked && Self::ref_type_of(gc, vector) == Type::T {
                    for index in 0..Self::ref_length(gc, vector) {
                        let value = Self::gc_ref(gc, env, vector, index).unwrap();

                        gc.mark(env, value)
                    }
                }
            }
        }
    }
}

// core
pub trait Core<'a> {
    fn as_string(_: &Env, _: Tag) -> String;
    fn evict(&self, _: &Env) -> Tag;
    fn from_string(_: &str) -> Vector;
    fn heap_size(_: &Env, _: Tag) -> usize;
    fn read(_: &Env, _: char, _: Tag) -> exception::Result<Tag>;
    fn ref_heap(_: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn gc_ref(_: &mut Gc, _: &Env, _: Tag, _: usize) -> Option<Tag>;
    fn view(_: &Env, _: Tag) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl<'a> Core<'a> for Vector {
    fn view(env: &Env, vector: Tag) -> Tag {
        let vec = vec![
            Fixnum::with_or_panic(Self::length(env, vector)),
            match Tag::type_key(Self::type_of(env, vector)) {
                Some(key) => key,
                None => panic!(),
            },
        ];

        TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn heap_size(env: &Env, vector: Tag) -> usize {
        match vector {
            Tag::Direct(_) => std::mem::size_of::<DirectTag>(),
            Tag::Indirect(_) => {
                let len = Self::length(env, vector);
                let size = match Self::type_of(env, vector) {
                    Type::Byte | Type::Char => 1,
                    Type::Fixnum | Type::Float | Type::T => 8,
                    _ => panic!(),
                };

                std::mem::size_of::<VectorImage>() + (size * len)
            }
        }
    }

    fn from_string(str: &str) -> Vector {
        let len = str.len();

        if len > DirectTag::DIRECT_STR_MAX {
            TypedVector::<String> {
                vec: str.to_string(),
            }
            .vec
            .to_vector()
        } else {
            let mut data: [u8; 8] = 0_u64.to_le_bytes();

            for (src, dst) in str.as_bytes().iter().zip(data.iter_mut()) {
                *dst = *src
            }

            Vector::Direct(DirectTag::to_direct(
                u64::from_le_bytes(data),
                DirectInfo::Length(len),
                DirectType::ByteVector,
            ))
        }
    }

    fn as_string(env: &Env, tag: Tag) -> String {
        match tag.type_of() {
            Type::Vector => match tag {
                Tag::Direct(dir) => match dir.dtype() {
                    DirectType::ByteVector => str::from_utf8(&dir.data().to_le_bytes()).unwrap()
                        [..dir.info() as usize]
                        .to_string(),
                    _ => panic!(),
                },
                Tag::Indirect(image) => {
                    let heap_ref = block_on(env.heap.read());
                    let vec: VectorImage = Self::to_image(env, tag);

                    str::from_utf8(
                        heap_ref
                            .image_data_slice(
                                image.image_id() as usize + Self::IMAGE_LEN,
                                0,
                                Fixnum::as_i64(vec.length) as usize,
                            )
                            .unwrap(),
                    )
                    .unwrap()
                    .to_string()
                }
            },
            _ => panic!(),
        }
    }

    fn write(env: &Env, vector: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match vector {
            Tag::Direct(_) => match str::from_utf8(&vector.data(env).to_le_bytes()) {
                Ok(s) => {
                    if escape {
                        env.write_string("\"", stream).unwrap()
                    }

                    for nth in 0..DirectTag::length(vector) {
                        Stream::write_char(env, stream, s.as_bytes()[nth] as char)?;
                    }

                    if escape {
                        env.write_string("\"", stream).unwrap()
                    }

                    Ok(())
                }
                Err(_) => panic!(),
            },
            Tag::Indirect(_) => match Self::type_of(env, vector) {
                Type::Char => {
                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    for ch in VectorIter::new(env, vector) {
                        env.write_stream(ch, false, stream)?;
                    }

                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    Ok(())
                }
                _ => {
                    env.write_string("#(", stream)?;
                    env.write_stream(Self::to_image(env, vector).vtype, true, stream)?;

                    for tag in VectorIter::new(env, vector) {
                        env.write_string(" ", stream)?;
                        env.write_stream(tag, false, stream)?;
                    }

                    env.write_string(")", stream)
                }
            },
        }
    }

    fn read(env: &Env, syntax: char, stream: Tag) -> exception::Result<Tag> {
        match syntax {
            '"' => {
                let mut str: String = String::new();

                loop {
                    match Stream::read_char(env, stream)? {
                        Some('"') => break,
                        Some(ch) => match map_char_syntax(ch).unwrap() {
                            SyntaxType::Escape => match Stream::read_char(env, stream)? {
                                Some(ch) => str.push(ch),
                                None => {
                                    return Err(Exception::new(
                                        env,
                                        Condition::Eof,
                                        "crux:read",
                                        stream,
                                    ));
                                }
                            },
                            _ => str.push(ch),
                        },
                        None => {
                            return Err(Exception::new(env, Condition::Eof, "crux:read", stream));
                        }
                    }
                }

                Ok(Self::from_string(&str).evict(env))
            }
            '(' => {
                let vec_list = match Cons::read(env, stream) {
                    Ok(list) => {
                        if list.null_() {
                            return Err(Exception::new(
                                env,
                                Condition::Type,
                                "crux:read",
                                Tag::nil(),
                            ));
                        }
                        list
                    }
                    Err(_) => {
                        return Err(Exception::new(env, Condition::Syntax, "crux:read", stream));
                    }
                };

                let vec_type = Cons::car(env, vec_list);

                match VTYPEMAP.iter().copied().find(|tab| vec_type.eq_(&tab.0)) {
                    Some(tab) => match tab.1 {
                        Type::T => {
                            let vec = Cons::iter(env, Cons::cdr(env, vec_list))
                                .map(|cons| Cons::car(env, cons))
                                .collect::<Vec<Tag>>();
                            Ok(TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env))
                        }
                        Type::Char => {
                            let vec: exception::Result<String> =
                                Cons::iter(env, Cons::cdr(env, vec_list))
                                    .map(|cons| {
                                        let ch = Cons::car(env, cons);
                                        if ch.type_of() == Type::Char {
                                            Ok(Char::as_char(env, ch))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "crux:read",
                                                ch,
                                            ))
                                        }
                                    })
                                    .collect();

                            Ok(TypedVector::<String> { vec: vec? }
                                .vec
                                .to_vector()
                                .evict(env))
                        }
                        Type::Byte => {
                            let vec: exception::Result<Vec<u8>> =
                                Cons::iter(env, Cons::cdr(env, vec_list))
                                    .map(|cons| {
                                        let fx = Cons::car(env, cons);
                                        if fx.type_of() == Type::Fixnum {
                                            let byte = Fixnum::as_i64(fx);
                                            if !(0..255).contains(&byte) {
                                                Err(Exception::new(
                                                    env,
                                                    Condition::Range,
                                                    "crux:read",
                                                    fx,
                                                ))
                                            } else {
                                                Ok(byte as u8)
                                            }
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "crux:read",
                                                fx,
                                            ))
                                        }
                                    })
                                    .collect();

                            Ok(TypedVector::<Vec<u8>> { vec: vec? }
                                .vec
                                .to_vector()
                                .evict(env))
                        }
                        Type::Fixnum => {
                            let vec: exception::Result<Vec<i64>> =
                                Cons::iter(env, Cons::cdr(env, vec_list))
                                    .map(|cons| {
                                        let fx = Cons::car(env, cons);
                                        if fx.type_of() == Type::Fixnum {
                                            Ok(Fixnum::as_i64(fx))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "crux:read",
                                                fx,
                                            ))
                                        }
                                    })
                                    .collect();

                            Ok(TypedVector::<Vec<i64>> { vec: vec? }
                                .vec
                                .to_vector()
                                .evict(env))
                        }
                        Type::Float => {
                            let vec: exception::Result<Vec<f32>> =
                                Cons::iter(env, Cons::cdr(env, vec_list))
                                    .map(|cons| {
                                        let fl = Cons::car(env, cons);
                                        if fl.type_of() == Type::Float {
                                            Ok(Float::as_f32(env, fl))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "crux:read",
                                                fl,
                                            ))
                                        }
                                    })
                                    .collect();

                            Ok(TypedVector::<Vec<f32>> { vec: vec? }
                                .vec
                                .to_vector()
                                .evict(env))
                        }
                        _ => panic!(),
                    },
                    None => Err(Exception::new(env, Condition::Type, "crux:read", vec_type)),
                }
            }
            _ => panic!(),
        }
    }

    fn evict(&self, env: &Env) -> Tag {
        match self {
            Vector::Direct(tag) => *tag,
            Vector::Indirect(image, ivec) => {
                let indirect = match ivec {
                    IVec::T(_) => IndirectVector::T(image, ivec),
                    IVec::Char(_) => IndirectVector::Char(image, ivec),
                    IVec::Byte(_) => IndirectVector::Byte(image, ivec),
                    IVec::Fixnum(_) => IndirectVector::Fixnum(image, ivec),
                    IVec::Float(_) => IndirectVector::Float(image, ivec),
                };

                match ivec {
                    IVec::T(_) => indirect.evict(env),
                    _ => match Self::cached(env, &indirect) {
                        Some(tag) => tag,
                        None => {
                            let tag = indirect.evict(env);

                            Self::cache(env, tag);
                            tag
                        }
                    },
                }
            }
        }
    }

    fn gc_ref(gc: &mut Gc, env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Direct(_direct) => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                Tag::Indirect(_) => IndirectVector::gc_ref(gc, vector, index),
            },
            _ => {
                panic!()
            }
        }
    }

    fn ref_heap(env: &Env, vector: Tag, index: usize) -> Option<Tag> {
        match vector.type_of() {
            Type::Vector => match vector {
                Tag::Direct(_direct) => {
                    let ch: char = vector.data(env).to_le_bytes()[index].into();

                    Some(ch.into())
                }
                Tag::Indirect(_) => IndirectVector::ref_heap(env, vector, index),
            },
            _ => {
                panic!()
            }
        }
    }
}

// env functions
pub trait CoreFunction {
    fn crux_type(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_length(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_make_vector(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_svref(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Vector {
    fn crux_make_vector(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let type_sym = fp.argv[0];
        let list = fp.argv[1];

        env.fp_argv_check("crux:make-vector", &[Type::Keyword, Type::List], fp)?;

        fp.value = match Self::to_type(type_sym) {
            Some(vtype) => match vtype {
                Type::Null => {
                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "crux:make-vector",
                        type_sym,
                    ))
                }
                Type::T => {
                    let vec = Cons::iter(env, list)
                        .map(|cons| Cons::car(env, cons))
                        .collect::<Vec<Tag>>();

                    TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
                }
                Type::Char => {
                    let vec: exception::Result<String> = Cons::iter(env, list)
                        .map(|cons| {
                            let ch = Cons::car(env, cons);
                            if ch.type_of() == Type::Char {
                                Ok(Char::as_char(env, ch))
                            } else {
                                Err(Exception::new(env, Condition::Type, "crux:make-vector", ch))
                            }
                        })
                        .collect();

                    TypedVector::<String> { vec: vec? }
                        .vec
                        .to_vector()
                        .evict(env)
                }
                Type::Byte => {
                    let vec: exception::Result<Vec<u8>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fx = Cons::car(env, cons);
                            if fx.type_of() == Type::Fixnum {
                                let byte = Fixnum::as_i64(fx);
                                if !(0..255).contains(&byte) {
                                    Err(Exception::new(
                                        env,
                                        Condition::Range,
                                        "crux:make-vector",
                                        fx,
                                    ))
                                } else {
                                    Ok(byte as u8)
                                }
                            } else {
                                Err(Exception::new(env, Condition::Type, "crux:make-vector", fx))
                            }
                        })
                        .collect();

                    TypedVector::<Vec<u8>> { vec: vec? }
                        .vec
                        .to_vector()
                        .evict(env)
                }
                Type::Fixnum => {
                    let vec: exception::Result<Vec<i64>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fx = Cons::car(env, cons);
                            if fx.type_of() == Type::Fixnum {
                                Ok(Fixnum::as_i64(fx))
                            } else {
                                Err(Exception::new(env, Condition::Type, "crux:make-vector", fx))
                            }
                        })
                        .collect();

                    TypedVector::<Vec<i64>> { vec: vec? }
                        .vec
                        .to_vector()
                        .evict(env)
                }
                Type::Float => {
                    let vec: exception::Result<Vec<f32>> = Cons::iter(env, list)
                        .map(|cons| {
                            let fl = Cons::car(env, cons);
                            if fl.type_of() == Type::Float {
                                Ok(Float::as_f32(env, fl))
                            } else {
                                Err(Exception::new(env, Condition::Type, "crux:make-vector", fl))
                            }
                        })
                        .collect();

                    TypedVector::<Vec<f32>> { vec: vec? }
                        .vec
                        .to_vector()
                        .evict(env)
                }
                _ => {
                    return Err(Exception::new(
                        env,
                        Condition::Type,
                        "crux:make-vector",
                        type_sym,
                    ));
                }
            },
            None => {
                return Err(Exception::new(
                    env,
                    Condition::Type,
                    "crux:make-vector",
                    type_sym,
                ));
            }
        };

        Ok(())
    }

    fn crux_svref(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];
        let index = fp.argv[1];

        env.fp_argv_check("crux:vector-ref", &[Type::Vector, Type::Fixnum], fp)?;

        let nth = Fixnum::as_i64(index);

        if nth < 0 || nth as usize >= Self::length(env, vector) {
            return Err(Exception::new(
                env,
                Condition::Range,
                "crux:vector-ref",
                index,
            ));
        }

        fp.value = match Self::ref_heap(env, vector, nth as usize) {
            Some(nth) => nth,
            None => panic!(),
        };

        Ok(())
    }

    fn crux_type(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];

        env.fp_argv_check("crux:vector-type", &[Type::Vector], fp)?;
        fp.value = match Tag::type_key(Self::type_of(env, vector)) {
            Some(key) => key,
            None => panic!(),
        };

        Ok(())
    }

    fn crux_length(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let vector = fp.argv[0];

        env.fp_argv_check("crux:vector-len", &[Type::Vector], fp)?;
        fp.value = Fixnum::with_or_panic(Self::length(env, vector));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
