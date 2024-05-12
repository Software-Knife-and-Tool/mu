//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream read functions
use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        lib::Lib,
        quasi::QuasiReader,
        reader::{Core as _, EOL},
        readtable::{map_char_syntax, SyntaxType},
        types::{Tag, Type},
    },
    types::{
        cons::{Cons, Core as _},
        core_stream::{Core as _, Stream},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};

pub trait Core {
    fn read_stream(&self, _: Tag, _: bool, _: Tag, _: bool) -> exception::Result<Tag>;
}

impl Core for Env {
    // read_stream:
    //
    //  returns:
    //     Err raise exception if I/O problem, syntax error, or end of file and !eofp
    //     Ok(eof_value) if end of file and eofp
    //     Ok(tag) if the read succeeded,
    //
    #[allow(clippy::only_used_in_recursion)]
    fn read_stream(
        &self,
        stream: Tag,
        eof_error_p: bool,
        eof_value: Tag,
        recursivep: bool,
    ) -> exception::Result<Tag> {
        if Lib::read_ws(self, stream)?.is_none() {
            return if eof_error_p {
                Err(Exception::new(self, Condition::Eof, "core:read", stream))
            } else {
                Ok(eof_value)
            };
        };

        match Stream::read_char(self, stream)? {
            None => {
                if eof_error_p {
                    Err(Exception::new(self, Condition::Eof, "core:read", stream))
                } else {
                    Ok(eof_value)
                }
            }
            Some(ch) => match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => Lib::read_atom(self, ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match Lib::sharpsign_macro(self, stream) {
                            Ok(Some(tag)) => Ok(tag),
                            Ok(None) => {
                                Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                            }
                            Err(e) => Err(e),
                        },
                        _ => Err(Exception::new(
                            self,
                            Condition::Type,
                            "core:read",
                            Tag::from(ch as i64),
                        )),
                    },
                    SyntaxType::Tmacro => match ch {
                        '`' => QuasiReader::read(self, false, stream, false),
                        '\'' => {
                            match Self::read_stream(self, stream, false, Tag::nil(), recursivep) {
                                Ok(tag) => Ok(Cons::vlist(self, &[Symbol::keyword("quote"), tag])),
                                Err(e) => Err(e),
                            }
                        }
                        '"' => match Vector::read(self, '"', stream) {
                            Ok(tag) => Ok(tag),
                            Err(e) => Err(e),
                        },
                        '(' => match Cons::read(self, stream) {
                            Ok(cons) => Ok(cons),
                            Err(e) => Err(e),
                        },
                        ')' => {
                            if recursivep {
                                Ok(*EOL)
                            } else {
                                Err(Exception::new(self, Condition::Syntax, "core:read", stream))
                            }
                        }
                        ';' => match Lib::read_comment(self, stream) {
                            Ok(_) => {
                                Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                            }
                            Err(e) => Err(e),
                        },
                        ',' => Err(Exception::new(
                            self,
                            Condition::Range,
                            "core:read",
                            Tag::from(ch),
                        )),
                        _ => Err(Exception::new(
                            self,
                            Condition::Range,
                            "core:read",
                            Tag::from(ch),
                        )),
                    },
                    _ => Err(Exception::new(
                        self,
                        Condition::Read,
                        "core:read",
                        Tag::from(ch),
                    )),
                },
                _ => Err(Exception::new(
                    self,
                    Condition::Read,
                    "core:read",
                    Tag::from(ch),
                )),
            },
        }
    }
}

pub trait CoreFunction {
    fn core_read(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn core_read(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match env.fp_argv_check("core:read", &[Type::Stream], fp) {
            Ok(_) => match Self::read_stream(env, stream, !eof_error_p.null_(), eof_value, false) {
                Ok(tag) => tag,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

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
