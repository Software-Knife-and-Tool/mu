//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! stream write functions
use crate::{
    core::{
        apply::Core as _,
        env::Env,
        exception::{self},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{
        char::{Char, Core as _},
        cons::{Cons, Core as _},
        core_stream::{Core as _, Stream},
        fixnum::{Core as _, Fixnum},
        float::{Core as _, Float},
        function::{Core as _, Function},
        indirect_vector::Core as _,
        namespace::{Core as _, Namespace},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::Vector,
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
            Type::Namespace => Namespace::write(self, tag, escape, stream),
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
            Stream::write_char(self, stream, ch)?;
        }

        Ok(())
    }
}

pub trait CoreFunction {
    fn mu_write(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Env {
    fn mu_write(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = fp.argv[0];
        let escape = fp.argv[1];
        let stream = fp.argv[2];

        env.fp_argv_check("mu:write", &[Type::T, Type::T, Type::Stream], fp)?;
        env.write_stream(fp.value, !escape.null_(), stream)?;

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
