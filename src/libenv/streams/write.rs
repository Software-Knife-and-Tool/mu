//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream write functions
#[allow(unused_imports)]
use crate::{
    core::{
        apply::{Core as _, LibFunction as _},
        compile::{Compile, LibFunction as _},
        dynamic::LibFunction as _,
        env::Env,
        exception::{self, Condition, Exception, LibFunction as _},
        frame::{Frame, LibFunction as _},
        gc::{Gc, LibFunction as _},
        heap::{Heap, LibFunction as _},
        namespace::{LibFunction as _, Namespace},
        qquote::QqReader,
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
    fn write_stream(&self, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_string(&self, _: &str, _: Tag) -> exception::Result<()>;
}

impl Core for Env {
    fn write_stream(&self, tag: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        if stream.type_of() != Type::Stream {
            panic!("{:?}", stream.type_of())
        }

        match tag.type_of() {
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

pub trait LibFunction {
    fn lib_write(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Env {
    fn lib_write(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let value = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        fp.value = match env.fp_argv_check("write", &[Type::T, Type::T, Type::Stream], fp) {
            Ok(_) => match env.write_stream(value, !escape.null_(), stream) {
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
