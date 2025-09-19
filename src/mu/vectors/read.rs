//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// vector reader
use crate::{
    core_::{
        env::Env,
        exception::{self, Condition, Exception},
        readtable::SyntaxType,
        tag::Tag,
        type_::Type,
    },
    streams::reader::StreamReader,
    types::{
        char::Char,
        cons::Cons,
        fixnum::Fixnum,
        float::Float,
        vector::{Vector, VTYPEMAP},
    },
};

pub trait Read {
    fn read(_: &Env, _: char, _: Tag) -> exception::Result<Tag>;
}

impl Read for Vector {
    fn read(env: &Env, syntax: char, stream: Tag) -> exception::Result<Tag> {
        match syntax {
            '"' => {
                let mut str: String = String::new();

                loop {
                    match StreamReader::read_char(env, stream)? {
                        Some('"') => break,
                        Some(ch) => match SyntaxType::map_char_syntax(ch).unwrap() {
                            SyntaxType::Escape => match StreamReader::read_char(env, stream)? {
                                Some(ch) => str.push(ch),
                                None => {
                                    Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                                }
                            },
                            _ => str.push(ch),
                        },
                        None => Err(Exception::new(env, Condition::Eof, "mu:read", stream))?,
                    }
                }

                Ok(Self::from(str).evict(env))
            }
            '*' => {
                let mut digits: String = String::new();

                loop {
                    match StreamReader::read_char(env, stream)? {
                        Some(ch) => match SyntaxType::map_char_syntax(ch).unwrap() {
                            SyntaxType::Whitespace | SyntaxType::Tmacro => {
                                StreamReader::unread_char(env, stream, ch)?;
                                break;
                            }
                            SyntaxType::Escape => match StreamReader::read_char(env, stream)? {
                                Some(ch) => {
                                    if ch == '0' || ch == '1' {
                                        digits.push(ch)
                                    } else {
                                        Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                                    }
                                }
                                None => {
                                    Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                                }
                            },
                            _ => {
                                if ch == '0' || ch == '1' {
                                    digits.push(ch)
                                } else {
                                    Err(Exception::new(env, Condition::Eof, "mu:read", stream))?
                                }
                            }
                        },
                        None => {
                            return Err(Exception::new(env, Condition::Eof, "mu:read", stream));
                        }
                    }
                }

                let mut vec = vec![0; digits.len().div_ceil(8)];
                let bvec = &mut vec;

                for (i, ch) in digits.chars().enumerate() {
                    if ch == '1' {
                        bvec[i / 8] |= (1_i8) << (7 - i % 8)
                    }
                }

                Ok(Self::from((vec, digits.len())).evict(env))
            }
            '(' => {
                let vec_list = match Cons::read(env, stream) {
                    Ok(list) => {
                        if list.null_() {
                            Err(Exception::new(env, Condition::Type, "mu:read", Tag::nil()))?
                        }
                        list
                    }
                    Err(_) => Err(Exception::new(env, Condition::Syntax, "mu:read", stream))?,
                };

                let (vec_type, vec) = Cons::destruct(env, vec_list);

                match VTYPEMAP.iter().copied().find(|tab| vec_type.eq_(&tab.0)) {
                    Some(tab) => match tab.1 {
                        Type::T => {
                            let vec = Cons::list_iter(env, vec).collect::<Vec<Tag>>();

                            Ok(Vector::from(vec).evict(env))
                        }
                        Type::Char => {
                            let vec: exception::Result<String> =
                                Cons::list_iter(env, Cons::destruct(env, vec_list).1)
                                    .map(|ch| {
                                        if ch.type_of() == Type::Char {
                                            Ok(Char::as_char(env, ch))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "mu:read",
                                                ch,
                                            ))?
                                        }
                                    })
                                    .collect();

                            Ok(Vector::from(vec?).evict(env))
                        }
                        Type::Byte => {
                            let vec: exception::Result<Vec<u8>> =
                                Cons::list_iter(env, Cons::destruct(env, vec_list).1)
                                    .map(|fx| {
                                        if fx.type_of() == Type::Fixnum {
                                            let byte = Fixnum::as_i64(fx);
                                            if !(0..=255).contains(&byte) {
                                                Err(Exception::new(
                                                    env,
                                                    Condition::Range,
                                                    "mu:read",
                                                    fx,
                                                ))?
                                            } else {
                                                Ok(byte as u8)
                                            }
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "mu:read",
                                                fx,
                                            ))?
                                        }
                                    })
                                    .collect();

                            Ok(Vector::from(vec?).evict(env))
                        }
                        Type::Fixnum => {
                            let vec: exception::Result<Vec<i64>> =
                                Cons::list_iter(env, Cons::destruct(env, vec_list).1)
                                    .map(|fx| {
                                        if fx.type_of() == Type::Fixnum {
                                            Ok(Fixnum::as_i64(fx))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "mu:read",
                                                fx,
                                            ))?
                                        }
                                    })
                                    .collect();

                            Ok(Vector::from(vec?).evict(env))
                        }
                        Type::Float => {
                            let vec: exception::Result<Vec<f32>> =
                                Cons::list_iter(env, Cons::destruct(env, vec_list).1)
                                    .map(|fl| {
                                        if fl.type_of() == Type::Float {
                                            Ok(Float::as_f32(env, fl))
                                        } else {
                                            Err(Exception::new(
                                                env,
                                                Condition::Type,
                                                "mu:read",
                                                fl,
                                            ))?
                                        }
                                    })
                                    .collect();

                            Ok(Vector::from(vec?).evict(env))
                        }
                        _ => panic!(),
                    },
                    None => Err(Exception::new(env, Condition::Type, "mu:read", vec_type))?,
                }
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_test() {
        assert!(true);
    }
}
