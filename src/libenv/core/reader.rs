//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env reader
use crate::{
    core::{
        env::Env,
        exception::{self, Condition, Exception},
        lib::Lib,
        readtable::{map_char_syntax, SyntaxType},
        types::{Tag, Type},
    },
    types::{
        fixnum::Fixnum,
        stream::{Core as _, Stream},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vectors::{Core as _, Vector},
    },
};

//
// read functions return:
//
//     Ok(Some(())) if the function succeeded,
//     Ok(None) if end of file
//     Err if stream or syntax error
//     errors propagate out of read()
//
pub trait Core {
    fn read_atom(_: &Env, _: char, _: Tag) -> exception::Result<Tag>;
    fn read_block_comment(_: &Env, _: Tag) -> exception::Result<Option<()>>;
    fn read_char_literal(_: &Env, _: Tag) -> exception::Result<Option<Tag>>;
    fn read_comment(_: &Env, _: Tag) -> exception::Result<Option<()>>;
    fn read_ws(_: &Env, _: Tag) -> exception::Result<Option<()>>;
    fn sharp_macro(_: &Env, _: Tag) -> exception::Result<Option<Tag>>;
    fn read_token(_: &Env, _: Tag) -> exception::Result<Option<String>>;
}

impl Core for Lib {
    //
    // read whitespace:
    //
    //    leave non-ws char at the head of the stream
    //    return None on end of file (not an error)
    //    return Err exception for stream error
    //    return Ok(Some(())) for ws consumed
    //
    fn read_ws(env: &Env, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(env, stream) {
                Ok(Some(ch)) => {
                    if let Some(stype) = map_char_syntax(ch) {
                        match stype {
                            SyntaxType::Whitespace => (),
                            _ => {
                                Stream::unread_char(env, stream, ch).unwrap();
                                break;
                            }
                        }
                    }
                }
                Ok(None) => return Ok(None),
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read comment till end of line:
    //
    //     return Err exception for stream error
    //     return Ok(Some(())) for comment consumed
    //
    fn read_comment(env: &Env, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(env, stream) {
                Ok(Some(ch)) => {
                    if ch == '\n' {
                        break;
                    }
                }
                Ok(None) => return Err(Exception::new(Condition::Eof, "read:;", stream)),
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read block comment
    //
    //     leave non-ws char at the head of the stream
    //     return Err exception for stream error
    //     return Ok(Some(())) for comment consumed
    //
    fn read_block_comment(env: &Env, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match Stream::read_char(env, stream) {
                Ok(Some(ch)) => {
                    if ch == '|' {
                        match Stream::read_char(env, stream) {
                            Ok(Some(ch)) => {
                                if ch == '#' {
                                    break;
                                }
                            }
                            Ok(None) => {
                                return Err(Exception::new(Condition::Eof, "read:#|", stream))
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                Ok(None) => return Err(Exception::new(Condition::Eof, "read:#|", stream)),
                Err(e) => return Err(e),
            }
        }

        Ok(Some(()))
    }

    // read token
    //
    //     return Err exception for stream error
    //     return Ok(Some(String))
    //
    fn read_token(env: &Env, stream: Tag) -> exception::Result<Option<String>> {
        let mut token = String::new();

        loop {
            match Stream::read_char(env, stream) {
                Ok(Some(ch)) => match map_char_syntax(ch) {
                    Some(stype) => match stype {
                        SyntaxType::Constituent => token.push(ch),
                        SyntaxType::Whitespace | SyntaxType::Tmacro => {
                            Stream::unread_char(env, stream, ch).unwrap();
                            break;
                        }
                        _ => return Err(Exception::new(Condition::Range, "read:tk", stream)),
                    },
                    None => return Err(Exception::new(Condition::Range, "read:tk", stream)),
                },
                Ok(None) => {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Some(token))
    }

    // read symbol or numeric literal:
    //
    //      leave non-ws char at the head of the stream
    //      return Some(tag) for successful read
    //      return Err exception for stream I/O error or unexpected eof
    //
    fn read_atom(env: &Env, ch: char, stream: Tag) -> exception::Result<Tag> {
        let mut token = String::new();

        token.push(ch);
        loop {
            match Stream::read_char(env, stream) {
                Ok(Some(ch)) => match map_char_syntax(ch) {
                    Some(stype) => match stype {
                        SyntaxType::Constituent => token.push(ch),
                        SyntaxType::Whitespace | SyntaxType::Tmacro => {
                            Stream::unread_char(env, stream, ch).unwrap();
                            break;
                        }
                        _ => {
                            return Err(Exception::new(Condition::Range, "read:at", Tag::from(ch)))
                        }
                    },
                    None => return Err(Exception::new(Condition::Range, "read:at", Tag::from(ch))),
                },
                Ok(None) => {
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        match token.parse::<i64>() {
            Ok(fx) => {
                if Fixnum::is_i56(fx) {
                    Ok(Tag::from(fx))
                } else {
                    Err(Exception::new(
                        Condition::Over,
                        "read:at",
                        Vector::from_string(&token).evict(env),
                    ))
                }
            }
            Err(_) => match token.parse::<f32>() {
                Ok(fl) => Ok(Tag::from(fl)),
                Err(_) => match Symbol::parse(env, token) {
                    Ok(sym) => Ok(sym),
                    Err(e) => Err(e),
                },
            },
        }
    }

    // read_char_literal returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn read_char_literal(env: &Env, stream: Tag) -> exception::Result<Option<Tag>> {
        match Stream::read_char(env, stream) {
            Ok(Some(ch)) => match Stream::read_char(env, stream) {
                Ok(Some(space)) => match map_char_syntax(space) {
                    Some(sp_type) => match sp_type {
                        SyntaxType::Whitespace => Ok(Some(Tag::from(ch))),
                        SyntaxType::Constituent => {
                            Stream::unread_char(env, stream, space).unwrap();
                            match Self::read_token(env, stream) {
                                Ok(Some(str)) => {
                                    let phrase = ch.to_string() + &str;
                                    match phrase.as_str() {
                                        "tab" => Ok(Some(Tag::from('\t'))),
                                        "linefeed" => Ok(Some(Tag::from('\n'))),
                                        "space" => Ok(Some(Tag::from(' '))),
                                        "page" => Ok(Some(Tag::from('\x0c'))),
                                        "return" => Ok(Some(Tag::from('\r'))),
                                        _ => Err(Exception::new(
                                            Condition::Type,
                                            "read:ch",
                                            Vector::from_string(&phrase).evict(env),
                                        )),
                                    }
                                }
                                Ok(None) => Err(Exception::new(Condition::Eof, "read:ch", stream)),
                                Err(e) => Err(e),
                            }
                        }
                        _ => {
                            Stream::unread_char(env, stream, space).unwrap();
                            Ok(Some(Tag::from(ch)))
                        }
                    },
                    None => Err(Exception::new(Condition::Syntax, "read:ch", stream)),
                },
                Ok(None) => Ok(Some(Tag::from(ch))),
                Err(e) => Err(e),
            },
            Ok(None) => Err(Exception::new(Condition::Eof, "read:ch", stream)),
            Err(e) => Err(e),
        }
    }

    // sharp_macro returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn sharp_macro(env: &Env, stream: Tag) -> exception::Result<Option<Tag>> {
        match Stream::read_char(env, stream) {
            Ok(Some(ch)) => match ch {
                ':' => match Stream::read_char(env, stream) {
                    Ok(Some(ch)) => match Self::read_atom(env, ch, stream) {
                        Ok(atom) => match atom.type_of() {
                            Type::Symbol => Ok(Some(atom)),
                            _ => Err(Exception::new(Condition::Type, "read:#", stream)),
                        },
                        Err(e) => Err(e),
                    },
                    Ok(None) => Err(Exception::new(Condition::Eof, "read:#", stream)),
                    Err(e) => Err(e),
                },
                '|' => match Self::read_block_comment(env, stream) {
                    Ok(_) => Ok(None),
                    Err(e) => Err(e),
                },
                '\\' => Self::read_char_literal(env, stream),
                'S' | 's' => match Struct::read(env, stream) {
                    Ok(tag) => Ok(Some(tag)),
                    Err(e) => Err(e),
                },
                '(' => match Vector::read(env, '(', stream) {
                    Ok(tag) => Ok(Some(tag)),
                    Err(e) => Err(e),
                },
                'x' => match Self::read_token(env, stream) {
                    Ok(token) => match token {
                        Some(hex) => match i64::from_str_radix(&hex, 16) {
                            Ok(fx) => {
                                if Fixnum::is_i56(fx) {
                                    Ok(Some(Tag::from(fx)))
                                } else {
                                    Err(Exception::new(
                                        Condition::Over,
                                        "read:#x",
                                        Vector::from_string(&hex).evict(env),
                                    ))
                                }
                            }
                            Err(_) => {
                                Err(Exception::new(Condition::Syntax, "read:#", Tag::from(ch)))
                            }
                        },
                        None => panic!(),
                    },
                    Err(_) => Err(Exception::new(Condition::Syntax, "read:#", Tag::from(ch))),
                },
                _ => Err(Exception::new(Condition::Type, "read:#", Tag::from(ch))),
            },
            Ok(None) => Err(Exception::new(Condition::Eof, "read:#", stream)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
