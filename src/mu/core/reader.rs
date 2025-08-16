//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// reader
use crate::{
    core::{
        apply::Apply as _,
        compile::Compile,
        direct::{DirectExt, DirectTag, DirectType},
        env::Env,
        exception::{self, Condition, Exception},
        quasi::QuasiReader,
        readtable::SyntaxType,
        types::{Tag, Type},
    },
    streams::reader::StreamReader,
    types::{cons::Cons, fixnum::Fixnum, struct_::Struct, symbol::Symbol, vector::Vector},
};

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
    fn read(&self, _: Tag, _: bool, _: Tag, _: bool) -> exception::Result<Tag>;
}

//
// read functions return:
//
//     Ok(Some(())) if the function succeeded,
//     Ok(None) if end of file
//     Err if stream or syntax error
//     errors propagate out of read()
//
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
            match StreamReader::read_char(self, stream)? {
                Some(ch) => {
                    if let Some(stype) = SyntaxType::map_char_syntax(ch) {
                        match stype {
                            SyntaxType::Whitespace => (),
                            _ => {
                                StreamReader::unread_char(self, stream, ch).unwrap();
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
            match StreamReader::read_char(self, stream)? {
                Some(ch) => {
                    if ch == '\n' {
                        break;
                    }
                }
                None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
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
            match StreamReader::read_char(self, stream)? {
                Some(ch) => {
                    if ch == '|' {
                        match StreamReader::read_char(self, stream)? {
                            Some(ch) => {
                                if ch == '#' {
                                    break;
                                }
                            }
                            None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
                        }
                    }
                }
                None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
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

        while let Some(ch) = StreamReader::read_char(self, stream)? {
            match SyntaxType::map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => token.push(ch),
                    SyntaxType::Whitespace | SyntaxType::Tmacro => {
                        StreamReader::unread_char(self, stream, ch).unwrap();
                        break;
                    }
                    _ => Err(Exception::new(self, Condition::Range, "mu:read", stream))?,
                },
                None => Err(Exception::new(self, Condition::Range, "mu:read", stream))?,
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

        while let Some(ch) = StreamReader::read_char(self, stream)? {
            match SyntaxType::map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => token.push(ch),
                    SyntaxType::Whitespace | SyntaxType::Tmacro => {
                        StreamReader::unread_char(self, stream, ch).unwrap();
                        break;
                    }
                    _ => Err(Exception::new(self, Condition::Range, "mu:read", ch.into()))?,
                },
                None => Err(Exception::new(self, Condition::Range, "mu:read", ch.into()))?,
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
                    ))?
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
        match StreamReader::read_char(self, stream)? {
            Some(ch) => match StreamReader::read_char(self, stream)? {
                Some(space) => match SyntaxType::map_char_syntax(space) {
                    Some(sp_type) => match sp_type {
                        SyntaxType::Whitespace => Ok(Some(ch.into())),
                        SyntaxType::Constituent => {
                            StreamReader::unread_char(self, stream, space).unwrap();
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
                                        ))?,
                                    }
                                }
                                None => {
                                    Err(Exception::new(self, Condition::Eof, "mu:read", stream))?
                                }
                            }
                        }
                        _ => {
                            StreamReader::unread_char(self, stream, space).unwrap();
                            Ok(Some(ch.into()))
                        }
                    },
                    None => Err(Exception::new(self, Condition::Syntax, "mu:read", stream))?,
                },
                None => Ok(Some(ch.into())),
            },
            None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
        }
    }

    // sharpsign_macro returns:
    //
    //     Err exception if I/O problem or syntax error
    //     Ok(tag) if the read succeeded,
    //
    fn sharpsign_macro(&self, stream: Tag) -> exception::Result<Option<Tag>> {
        match StreamReader::read_char(self, stream)? {
            Some(ch) => match ch {
                ':' => match StreamReader::read_char(self, stream)? {
                    Some(ch) => {
                        let atom = Self::read_atom(self, ch, stream)?;

                        match atom.type_of() {
                            Type::Symbol => Ok(Some(atom)),
                            _ => Err(Exception::new(self, Condition::Type, "mu:read", stream))?,
                        }
                    }
                    None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
                },
                '.' => Ok(Some(self.eval(self.read(
                    stream,
                    false,
                    Tag::nil(),
                    false,
                )?)?)),
                '|' => {
                    Self::read_block_comment(self, stream)?;

                    Ok(None)
                }
                '\\' => Self::read_char_literal(self, stream),
                'S' | 's' => Ok(Some(Struct::read(self, stream)?)),
                '(' | '*' => Ok(Some(Vector::read(self, ch, stream)?)),
                'x' => match Self::read_token(self, stream) {
                    Ok(token) => {
                        let hex = token.unwrap();

                        match i64::from_str_radix(&hex, 16) {
                            Ok(fx) => {
                                if Fixnum::is_i56(fx) {
                                    Ok(Some(Fixnum::with_i64_or_panic(fx)))
                                } else {
                                    Err(Exception::new(
                                        self,
                                        Condition::Over,
                                        "mu:read",
                                        Vector::from(hex).evict(self),
                                    ))?
                                }
                            }
                            Err(_) => Err(Exception::new(
                                self,
                                Condition::Syntax,
                                "mu:read",
                                ch.into(),
                            ))?,
                        }
                    }
                    Err(_) => Err(Exception::new(
                        self,
                        Condition::Syntax,
                        "mu:read",
                        ch.into(),
                    ))?,
                },
                _ => Err(Exception::new(self, Condition::Type, "mu:read", ch.into()))?,
            },
            None => Err(Exception::new(self, Condition::Eof, "mu:read", stream))?,
        }
    }

    // read:
    //
    //  returns:
    //     Err raise exception if I/O problem, syntax error, or end of file and !eofp
    //     Ok(eof_value) if end of file and eofp
    //     Ok(tag) if the read succeeded,
    //
    fn read(
        &self,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
        recursivep: bool,
    ) -> exception::Result<Tag> {
        assert_eq!(stream.type_of(), Type::Stream);

        if self.read_ws(stream)?.is_none() {
            return if eof_error_p {
                Err(Exception::new(self, Condition::Eof, "reader", stream))
            } else {
                Ok(eof_value)
            };
        };

        match StreamReader::read_char(self, stream)? {
            None => {
                if eof_error_p {
                    Err(Exception::new(self, Condition::Eof, "mu:read", stream))?
                } else {
                    Ok(eof_value)
                }
            }
            Some(ch) => match SyntaxType::map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => self.read_atom(ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match self.sharpsign_macro(stream)? {
                            Some(tag) => Ok(tag),
                            None => self.read(stream, eof_error_p, eof_value, recursivep),
                        },
                        _ => Err(Exception::new(self, Condition::Type, "reader", ch.into()))?,
                    },
                    SyntaxType::Tmacro => match ch {
                        '`' => QuasiReader::read(self, false, stream, false),
                        '\'' => Ok(Compile::quote(
                            self,
                            &self.read(stream, false, Tag::nil(), recursivep)?,
                        )),
                        '"' => Ok(Vector::read(self, '"', stream)?),
                        '(' => Ok(Cons::read(self, stream)?),
                        ')' => {
                            if recursivep {
                                Ok(*EOL)
                            } else {
                                Err(Exception::new(self, Condition::Syntax, "reader", stream))?
                            }
                        }
                        ';' => {
                            self.read_comment(stream)?;
                            self.read(stream, eof_error_p, eof_value, recursivep)
                        }
                        ',' => Err(Exception::new(self, Condition::Range, "reader", ch.into()))?,
                        _ => Err(Exception::new(self, Condition::Range, "reader", ch.into()))?,
                    },
                    _ => Err(Exception::new(self, Condition::Read, "reader", ch.into()))?,
                },
                _ => Err(Exception::new(self, Condition::Read, "reader", ch.into()))?,
            },
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
