//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// core writer
use crate::{
    core::{
        env::Env,
        exception::{self},
        types::{Tag, Type},
    },
    types::{
        async_::Async, char::Char, cons::Cons, fixnum::Fixnum, float::Float, function::Function,
        stream::Stream, struct_::Struct, symbol::Symbol, vector::Vector,
    },
};

pub trait Writer {
    fn write(&self, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Writer for Env {
    fn write(&self, tag: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        assert_eq!(stream.type_of(), Type::Stream);

        match tag.type_of() {
            Type::Async => Async::write(self, tag, escape, stream),
            Type::Char => Char::write(self, tag, escape, stream),
            Type::Cons => Cons::write(self, tag, escape, stream),
            Type::Fixnum => Fixnum::write(self, tag, escape, stream),
            Type::Float => Float::write(self, tag, escape, stream),
            Type::Function => Function::write(self, tag, escape, stream),
            Type::Stream => Stream::write(self, tag, escape, stream),
            Type::Struct => Struct::write(self, tag, escape, stream),
            Type::Symbol | Type::Null | Type::Keyword => Symbol::write(self, tag, escape, stream),
            Type::Vector => Vector::write(self, tag, escape, stream),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn writer() {
        assert!(true);
    }
}
