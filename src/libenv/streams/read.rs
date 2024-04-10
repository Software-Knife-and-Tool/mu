//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream read functions
#[allow(unused_imports)]
use crate::{
    async_::context::{Context, Core as _, LibFunction as _},
    core::{
        apply::{Core as _, LibFunction as _},
        compile::{Compile, LibFunction as _},
        dynamic::LibFunction as _,
        env::Env,
        exception::{self, Condition, Exception, LibFunction as _},
        frame::{Frame, LibFunction as _},
        gc::{Gc, LibFunction as _},
        heap::{Heap, LibFunction as _},
        lib::Lib,
        lib::LIB,
        namespace::{LibFunction as _, Namespace},
        qquote::QqReader,
        reader::Core as _,
        reader::Core as _,
        readtable::{map_char_syntax, SyntaxType},
        types::{LibFunction as _, Tag, Type},
        utime::LibFunction as _,
    },
    types::{
        char::{Char, Core as _},
        cons::{Cons, Core as _, LibFunction as _},
        fixnum::{Core as _, Fixnum, LibFunction as _},
        float::{Core as _, Float, LibFunction as _},
        function::{Core as _, Function},
        stream::{Core as _, Stream},
        streams::LibFunction as _,
        struct_::{Core as _, LibFunction as _, Struct},
        symbol::{Core as _, LibFunction as _, Symbol, UNBOUND},
        vector::{Core as _, LibFunction as _, Vector},
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
        match Lib::read_ws(self, stream) {
            Ok(None) => {
                return if eof_error_p {
                    Err(Exception::new(Condition::Eof, "read", stream))
                } else {
                    Ok(eof_value)
                }
            }
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        match Stream::read_char(self, stream) {
            Ok(None) => {
                if eof_error_p {
                    Err(Exception::new(Condition::Eof, "read", stream))
                } else {
                    Ok(eof_value)
                }
            }
            Ok(Some(ch)) => match map_char_syntax(ch) {
                Some(stype) => match stype {
                    SyntaxType::Constituent => Lib::read_atom(self, ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match Lib::sharp_macro(self, stream) {
                            Ok(Some(tag)) => Ok(tag),
                            Ok(None) => {
                                Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                            }
                            Err(e) => Err(e),
                        },
                        _ => Err(Exception::new(
                            Condition::Type,
                            "read",
                            Tag::from(ch as i64),
                        )),
                    },
                    SyntaxType::Tmacro => match ch {
                        '`' => QqReader::read(self, false, stream, false),
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
                                Ok(LIB.eol)
                            } else {
                                Err(Exception::new(Condition::Syntax, "read", stream))
                            }
                        }
                        ';' => match Lib::read_comment(self, stream) {
                            Ok(_) => {
                                Self::read_stream(self, stream, eof_error_p, eof_value, recursivep)
                            }
                            Err(e) => Err(e),
                        },
                        ',' => Err(Exception::new(Condition::Range, "read", Tag::from(ch))),
                        _ => Err(Exception::new(Condition::Range, "read", Tag::from(ch))),
                    },
                    _ => Err(Exception::new(Condition::Read, "read", Tag::from(ch))),
                },
                _ => Err(Exception::new(Condition::Read, "read", Tag::from(ch))),
            },
            Err(e) => Err(e),
        }
    }
}

pub trait LibFunction {
    fn lib_read(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Env {
    fn lib_read(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match env.fp_argv_check("read", &[Type::Stream], fp) {
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