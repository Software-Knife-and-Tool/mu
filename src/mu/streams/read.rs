//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream read functions
use crate::{
    core::{
        apply::Apply as _,
        compile::Compile,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        quasi::QuasiReader,
        reader::{Reader, EOL},
        readtable::{map_char_syntax, SyntaxType},
        types::{Tag, Type},
    },
    types::{cons::Cons, stream::Read as _, vector::Vector},
};

pub trait Read {
    fn read_stream(&self, _: Tag, _: bool, _: Tag, _: bool) -> exception::Result<Tag>;
}

impl Read for Env {
    // read_stream:
    //
    //  returns:
    //     Err raise exception if I/O problem, syntax error, or end of file and !eofp
    //     Ok(eof_value) if end of file and eofp
    //     Ok(tag) if the read succeeded,
    //
    fn read_stream(
        &self,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
        recursivep: bool,
    ) -> exception::Result<Tag> {
        if self.read_ws(stream)?.is_none() {
            return if eof_error_p {
                Err(Exception::new(self, Condition::Eof, "mu:read", stream))
            } else {
                Ok(eof_value)
            };
        };

        match self.read_char(stream)? {
            None => {
                if eof_error_p {
                    Err(Exception::new(self, Condition::Eof, "mu:read", stream))
                } else {
                    Ok(eof_value)
                }
            }
            Some(ch) => match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => self.read_atom(ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match self.sharpsign_macro(stream)? {
                            Some(tag) => Ok(tag),
                            None => {
                                Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                            }
                        },
                        _ => Err(Exception::new(self, Condition::Type, "mu:read", ch.into())),
                    },
                    SyntaxType::Tmacro => match ch {
                        '`' => QuasiReader::read(self, false, stream, false),
                        '\'' => {
                            let tag =
                                Self::read_stream(self, stream, false, Tag::nil(), recursivep)?;

                            Ok(self.quote(&tag))
                        }
                        '"' => Ok(Vector::read(self, '"', stream)?),
                        '(' => Ok(Cons::read(self, stream)?),
                        ')' => {
                            if recursivep {
                                Ok(*EOL)
                            } else {
                                Err(Exception::new(self, Condition::Syntax, "mu:read", stream))
                            }
                        }
                        ';' => {
                            self.read_comment(stream)?;

                            Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                        }
                        ',' => Err(Exception::new(self, Condition::Range, "mu:read", ch.into())),
                        _ => Err(Exception::new(self, Condition::Range, "mu:read", ch.into())),
                    },
                    _ => Err(Exception::new(self, Condition::Read, "mu:read", ch.into())),
                },
                _ => Err(Exception::new(self, Condition::Read, "mu:read", ch.into())),
            },
        }
    }
}

pub trait CoreFunction {
    fn mu_read(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_read(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        env.fp_argv_check("mu:read", &[Type::Stream], fp)?;
        fp.value = Self::read_stream(env, stream, !eof_error_p.null_(), eof_value, false)?;

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
