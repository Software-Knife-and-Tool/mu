//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu system functions
#[allow(unused_imports)]
use crate::{
    async_::context::{Context, Core as _},
    core::{
        apply::Core as _,
        exception::{self, Condition, Exception},
        frame::Frame,
        mu::Mu,
        qquote::QqReader,
        reader::{Core as _, Reader},
        readtable::{map_char_syntax, SyntaxType},
        types::{Tag, Type},
    },
    types::{
        char::{Char, Core as _},
        cons::{Cons, Core as _},
        fixnum::{Core as _, Fixnum},
        float::{Core as _, Float},
        function::{Core as _, Function},
        stream::{Core as _, Stream},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::{Core as _, Vector},
    },
};

pub trait Core {
    fn debug_vprintln(&self, _: &str, _: bool, _: Tag);
    fn debug_vprint(&self, _: &str, _: bool, _: Tag);

    fn read_stream(&self, _: Tag, _: bool, _: Tag, _: bool) -> exception::Result<Tag>;
    fn write_stream(&self, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_string(&self, _: &str, _: Tag) -> exception::Result<()>;
}

impl Core for Mu {
    // debug printing
    //
    fn debug_vprint(&self, label: &str, verbose: bool, tag: Tag) {
        print!("{}: ", label);
        self.write_stream(tag, verbose, self.stdout).unwrap();
    }

    fn debug_vprintln(&self, label: &str, verbose: bool, tag: Tag) {
        self.debug_vprint(label, verbose, tag);
        println!();
    }

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
        match Reader::read_ws(self, stream) {
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
                    SyntaxType::Constituent => Reader::read_atom(self, ch, stream),
                    SyntaxType::Macro => match ch {
                        '#' => match Reader::sharp_macro(self, stream) {
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
                                Ok(self.reader.eol)
                            } else {
                                Err(Exception::new(Condition::Syntax, "read", stream))
                            }
                        }
                        ';' => match Reader::read_comment(self, stream) {
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

    fn write_stream(&self, tag: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        if stream.type_of() != Type::Stream {
            panic!("{:?}", stream.type_of())
        }

        match tag.type_of() {
            Type::AsyncId => Context::write(self, tag, escape, stream),
            Type::Char => Char::write(self, tag, escape, stream),
            Type::Cons => Cons::write(self, tag, escape, stream),
            Type::Fixnum => Fixnum::write(self, tag, escape, stream),
            Type::Float => Float::write(self, tag, escape, stream),
            Type::Function => Function::write(self, tag, escape, stream),
            Type::Keyword => Symbol::write(self, tag, escape, stream),
            Type::Null => Symbol::write(self, tag, escape, stream),
            Type::Stream => Stream::write(self, tag, escape, stream),
            Type::Struct => Struct::write(self, tag, escape, stream),
            Type::Symbol => Symbol::write(self, tag, escape, stream),
            Type::Vector => Vector::write(self, tag, escape, stream),
            _ => panic!(),
        }
    }

    fn write_string(&self, str: &str, stream: Tag) -> exception::Result<()> {
        if stream.type_of() != Type::Stream {
            panic!("{:?}", stream.type_of())
        }

        for ch in str.chars() {
            match Stream::write_char(self, stream, ch) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

pub trait MuFunction {
    fn core_read(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn core_write(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Mu {
    fn core_read(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let stream = fp.argv[0];
        let eof_error_p = fp.argv[1];
        let eof_value = fp.argv[2];

        fp.value = match mu.fp_argv_check("read", &[Type::Stream], fp) {
            Ok(_) => match Self::read_stream(mu, stream, !eof_error_p.null_(), eof_value, false) {
                Ok(tag) => tag,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_write(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let value = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        fp.value = match mu.fp_argv_check("write", &[Type::T, Type::T, Type::Stream], fp) {
            Ok(_) => match mu.write_stream(value, !escape.null_(), stream) {
                Ok(_) => value,
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
