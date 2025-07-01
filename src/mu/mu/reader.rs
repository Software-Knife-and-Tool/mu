//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env reader
use crate::{
    mu::{
        apply::Apply as _,
        direct::{DirectExt, DirectTag, DirectType},
        env::Env,
        exception::{self, Condition, Exception},
        readtable::{map_char_syntax, SyntaxType},
        types::{Tag, Type},
    },
    streams::read::Read as _,
    types::{fixnum::Fixnum, stream::Read, struct_::Struct, symbol::Symbol, vector::Vector},
};

//
// read functions return:
//
//     Ok(Some(())) if the function succeeded,
//     Ok(None) if end of file
//     Err if stream or syntax error
//     errors propagate out of read()
//

lazy_static! {
    pub static ref EOL: Tag = DirectTag::to_tag(0, DirectExt::Length(0), DirectType::Keyword);
}

pub trait Reader {
    fn read_atom(&self, _: char, _: Tag) -> exception::Result<Tag>;
    fn read_block_comment(&self, _: Tag) -> exception::Result<Option<()>>;
    fn read_char_literal(&self, _: Tag) -> exception::Result<Option<Tag>>;
    fn read_comment(&self, _: Tag) -> exception::Result<Option<()>>;
    fn read_ws(&self, _: Tag) -> exception::Result<Option<()>>;
    fn sharpsign_macro(&self, _: Tag) -> exception::Result<Option<Tag>>;
    fn read_token(&self, _: Tag) -> exception::Result<Option<String>>;
}

impl Reader for Env {
    //
    // read whitespace:
    //
    //    leave non-ws char at the head of the stream
    //    return None on end of file (not an error)
    //    return Err exception for stream error
    //    return Ok(Some(())) for ws consumed
    //
    fn read_ws(&self, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match self.read_char(stream)? {
                Some(ch) => {
                    if let Some(stype) = map_char_syntax(ch) {
                        match stype {
                            SyntaxType::Whitespace => (),
                            _ => {
                                self.unread_char(stream, ch).unwrap();
                                break;
                            }
                        }
                    }
                }
                None => return Ok(None),
            }
        }

        Ok(Some(()))
    }

    // read comment till end of line:
    //
    //     return Err exception for stream error
    //     return Ok(Some(())) for comment consumed
    //
    fn read_comment(&self, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match self.read_char(stream)? {
                Some(ch) => {
                    if ch == '\n' {
                        break;
                    }
                }
                None => return Err(Exception::new(self, Condition::Eof, "mu:read", stream)),
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
    fn read_block_comment(&self, stream: Tag) -> exception::Result<Option<()>> {
        loop {
            match self.read_char(stream)? {
                Some(ch) => {
                    if ch == '|' {
                        match self.read_char(stream)? {
                            Some(ch) => {
                                if ch == '#' {
                                    break;
                                }
                            }
                            None => {
                                return Err(Exception::new(self, Condition::Eof, "mu:read", stream))
                            }
                        }
                    }
                }
                None => return Err(Exception::new(self, Condition::Eof, "mu:read", stream)),
            }
        }

        Ok(Some(()))
    }

    // read token
    //
    //     return Err exception for stream error
    //     return Ok(Some(String))
    //
    fn read_token(&self, stream: Tag) -> exception::Result<Option<String>> {
        let mut token = String::new();

        while let Some(ch) = self.read_char(stream)? {
            match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => token.push(ch),
                    SyntaxType::Whitespace | SyntaxType::Tmacro => {
                        self.unread_char(stream, ch).unwrap();
                        break;
                    }
                    _ => return Err(Exception::new(self, Condition::Range, "mu:read", stream)),
                },
                None => return Err(Exception::new(self, Condition::Range, "mu:read", stream)),
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
    fn read_atom(&self, ch: char, stream: Tag) -> exception::Result<Tag> {
        let mut token = String::new();

        token.push(ch);

        while let Some(ch) = self.read_char(stream)? {
            match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => token.push(ch),
                    SyntaxType::Whitespace | SyntaxType::Tmacro => {
                        self.unread_char(stream, ch).unwrap();
                        break;
                    }
                    _ => return Err(Exception::new(self, Condition::Range, "mu:read", ch.into())),
                },
                None => return Err(Exception::new(self, Condition::Range, "mu:read", ch.into())),
            }
        }

        match token.parse::<i64>() {
            Ok(fx) => {
                if Fixnum::is_i56(fx) {
                    Ok(Fixnum::with_i64_or_panic(fx))
                } else {
                    Err(Exception::new(
                        self,
                        Condition::Over,
                        "mu:read",
                        Vector::from(token).evict(self),
                    ))
                }
            }
            Err(_) => match token.parse::<f32>() {
                Ok(fl) => Ok(fl.into()),
                Err(_) => Ok(Symbol::parse(self, token)?),
            },
        }
    }

    // read_char_literal returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn read_char_literal(&self, stream: Tag) -> exception::Result<Option<Tag>> {
        match self.read_char(stream)? {
            Some(ch) => match self.read_char(stream)? {
                Some(space) => match map_char_syntax(space) {
                    Some(sp_type) => match sp_type {
                        SyntaxType::Whitespace => Ok(Some(ch.into())),
                        SyntaxType::Constituent => {
                            self.unread_char(stream, space).unwrap();
                            match Self::read_token(self, stream)? {
                                Some(str) => {
                                    let phrase = ch.to_string() + &str;
                                    match phrase.as_str() {
                                        "tab" => Ok(Some('\t'.into())),
                                        "linefeed" => Ok(Some('\n'.into())),
                                        "space" => Ok(Some(' '.into())),
                                        "page" => Ok(Some('\x0c'.into())),
                                        "return" => Ok(Some('\r'.into())),
                                        _ => Err(Exception::new(
                                            self,
                                            Condition::Type,
                                            "mu:read",
                                            Vector::from(phrase).evict(self),
                                        )),
                                    }
                                }
                                None => {
                                    Err(Exception::new(self, Condition::Eof, "mu:read", stream))
                                }
                            }
                        }
                        _ => {
                            self.unread_char(stream, space).unwrap();
                            Ok(Some(ch.into()))
                        }
                    },
                    None => Err(Exception::new(self, Condition::Syntax, "mu:read", stream)),
                },
                None => Ok(Some(ch.into())),
            },
            None => Err(Exception::new(self, Condition::Eof, "mu:read", stream)),
        }
    }

    // sharpsign_macro returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn sharpsign_macro(&self, stream: Tag) -> exception::Result<Option<Tag>> {
        match self.read_char(stream)? {
            Some(ch) => match ch {
                ':' => match self.read_char(stream)? {
                    Some(ch) => {
                        let atom = Self::read_atom(self, ch, stream)?;

                        match atom.type_of() {
                            Type::Symbol => Ok(Some(atom)),
                            _ => Err(Exception::new(self, Condition::Type, "mu:read", stream)),
                        }
                    }
                    None => Err(Exception::new(self, Condition::Eof, "mu:read", stream)),
                },
                '.' => {
                    let expr = self.read_stream(stream, false, Tag::nil(), false)?;

                    Ok(Some(self.eval(expr)?))
                }
                '|' => {
                    Self::read_block_comment(self, stream)?;

                    Ok(None)
                }
                '\\' => Self::read_char_literal(self, stream),
                'S' | 's' => Ok(Some(Struct::read(self, stream)?)),
                '(' | '*' => Ok(Some(Vector::read(self, ch, stream)?)),
                'x' => match Self::read_token(self, stream) {
                    Ok(token) => match token {
                        Some(hex) => match i64::from_str_radix(&hex, 16) {
                            Ok(fx) => {
                                if Fixnum::is_i56(fx) {
                                    Ok(Some(Fixnum::with_i64_or_panic(fx)))
                                } else {
                                    Err(Exception::new(
                                        self,
                                        Condition::Over,
                                        "mu:read",
                                        Vector::from(hex).evict(self),
                                    ))
                                }
                            }
                            Err(_) => Err(Exception::new(
                                self,
                                Condition::Syntax,
                                "mu:read",
                                ch.into(),
                            )),
                        },
                        None => panic!(),
                    },
                    Err(_) => Err(Exception::new(
                        self,
                        Condition::Syntax,
                        "mu:read",
                        ch.into(),
                    )),
                },
                /*
                    '!' => {
                        let hexstr = (0..=15)
                            .map(|_| match self.read_char(stream).unwrap() {
                                Some(ch) => ch,
                                None => panic!(),
                            })
                            .collect::<String>();
                        match u64::from_str_radix(&hexstr, 16) {
                            Ok(tag_bits) => Ok(Some((&tag_bits.to_be_bytes()).into())),
                            Err(_) => Err(Exception::new(
                                self,
                                Condition::Syntax,
                                "mu:read",
                                ch.into(),
                            )),
                        }
                }
                    */
                _ => Err(Exception::new(self, Condition::Type, "mu:read", ch.into())),
            },
            None => Err(Exception::new(self, Condition::Eof, "mu:read", stream)),
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
