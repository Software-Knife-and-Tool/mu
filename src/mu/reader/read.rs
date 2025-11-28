//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// reader
#[rustfmt::skip]
use {
    crate::{
        core::{
            apply::Apply as _,
            compiler::Compiler,
            direct::{DirectExt, DirectTag, DirectType},
            env::Env,
            exception::{self, Condition, Exception},
            tag::{Tag},
            type_::{Type},
        },
        reader::{quasi::QuasiReader, readtable::SyntaxType},
        streams::reader::StreamReader,
        types::{
            cons::Cons,
            fixnum::Fixnum,
            struct_::Struct,
            symbol::{Symbol, SymbolImage},
            vector::Vector
        },
    },
    std::sync::LazyLock,
};

pub static EOL: LazyLock<Tag> =
    LazyLock::new(|| DirectTag::to_tag(0, DirectExt::Length(0), DirectType::Keyword));

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
                        if stype != &SyntaxType::Whitespace {
                            StreamReader::unread_char(self, stream, ch).unwrap();
                            break;
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
                None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
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
                            None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
                        }
                    }
                }
                None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
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
                    _ => Err(Exception::err(self, stream, Condition::Range, "mu:read"))?,
                },
                None => Err(Exception::err(self, stream, Condition::Range, "mu:read"))?,
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
                    _ => Err(Exception::err(self, ch.into(), Condition::Range, "mu:read"))?,
                },
                None => Err(Exception::err(self, ch.into(), Condition::Range, "mu:read"))?,
            }
        }

        match token.parse::<i64>() {
            Ok(fx) => {
                if Fixnum::is_i56(fx) {
                    Ok(Fixnum::with_i64(self, fx).unwrap())
                } else {
                    Err(Exception::err(
                        self,
                        Vector::from(token).with_heap(self),
                        Condition::Over,
                        "mu:read",
                    ))?
                }
            }
            Err(_) => match token.parse::<f32>() {
                Ok(fl) => Ok(fl.into()),
                Err(_) => Ok(Symbol::parse(self, &token)?),
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
                                        _ => Err(Exception::err(
                                            self,
                                            Vector::from(phrase).with_heap(self),
                                            Condition::Type,
                                            "mu:read",
                                        ))?,
                                    }
                                }
                                None => {
                                    Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?
                                }
                            }
                        }
                        _ => {
                            StreamReader::unread_char(self, stream, space).unwrap();
                            Ok(Some(ch.into()))
                        }
                    },
                    None => Err(Exception::err(self, stream, Condition::Syntax, "mu:read"))?,
                },
                None => Ok(Some(ch.into())),
            },
            None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
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
                            Type::Symbol => {
                                let (namespace, name, value) = Symbol::destruct(self, atom);

                                if namespace.null_() {
                                    Err(Exception::err(
                                        self,
                                        namespace,
                                        Condition::Type,
                                        "mu:read",
                                    ))?;
                                }

                                let symbol = Symbol::Symbol(SymbolImage {
                                    namespace: Tag::nil(),
                                    name,
                                    value,
                                })
                                .with_heap(self);

                                Ok(Some(symbol))
                            }
                            _ => Err(Exception::err(self, atom, Condition::Type, "mu:read"))?,
                        }
                    }
                    None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
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
                                    Ok(Some(Fixnum::with_i64(self, fx).unwrap()))
                                } else {
                                    Err(Exception::err(
                                        self,
                                        Vector::from(hex).with_heap(self),
                                        Condition::Over,
                                        "mu:read",
                                    ))?
                                }
                            }
                            Err(_) => Err(Exception::err(
                                self,
                                ch.into(),
                                Condition::Syntax,
                                "mu:read",
                            ))?,
                        }
                    }
                    Err(_) => Err(Exception::err(
                        self,
                        ch.into(),
                        Condition::Syntax,
                        "mu:read",
                    ))?,
                },
                _ => Err(Exception::err(self, ch.into(), Condition::Type, "mu:read"))?,
            },
            None => Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?,
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
                Err(Exception::err(self, stream, Condition::Eof, "reader"))
            } else {
                Ok(eof_value)
            };
        }

        match StreamReader::read_char(self, stream)? {
            None => {
                if eof_error_p {
                    Err(Exception::err(self, stream, Condition::Eof, "mu:read"))?
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
                        _ => Err(Exception::err(self, ch.into(), Condition::Type, "reader"))?,
                    },
                    SyntaxType::Tmacro => match ch {
                        '`' => QuasiReader::read(self, false, stream, false),
                        '\'' => Ok(Compiler::quote(
                            self,
                            &self.read(stream, false, Tag::nil(), recursivep)?,
                        )),
                        '"' => Ok(Vector::read(self, '"', stream)?),
                        '(' => Ok(Cons::read(self, stream)?),
                        ')' => {
                            if recursivep {
                                Ok(*EOL)
                            } else {
                                Err(Exception::err(self, stream, Condition::Syntax, "reader"))?
                            }
                        }
                        ';' => {
                            self.read_comment(stream)?;
                            self.read(stream, eof_error_p, eof_value, recursivep)
                        }
                        _ => Err(Exception::err(self, ch.into(), Condition::Range, "reader"))?,
                    },
                    _ => Err(Exception::err(self, ch.into(), Condition::Read, "reader"))?,
                },
                _ => Err(Exception::err(self, ch.into(), Condition::Read, "reader"))?,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reader_test() {
        assert!(true);
    }
}
