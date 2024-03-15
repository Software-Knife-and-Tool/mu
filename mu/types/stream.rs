//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu stream type
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::identity_op)]

use crate::{
    core::{
        direct::{DirectInfo, DirectTag, DirectType, ExtType},
        exception::{self, Condition, Exception},
        mu::Mu,
        stream::{Core as _, SystemStream},
        system::Core as _,
        types::{Tag, Type},
    },
    types::{
        char::Char,
        symbol::{Core as _, Symbol},
        vecimage::{TypedVec, VecType},
        vector::Core as _,
    },
};

use futures::executor::block_on;

// stream struct
pub struct Stream {
    pub system: SystemStream, // system stream
    pub index: usize,         // stream table index
    pub open: bool,           // stream open
    pub direction: Tag,       // :input | :output | :bidir (keyword)
    pub unch: Tag,            // pushbask for input streams
}

impl From<Stream> for Tag {
    fn from(stream: Stream) -> Tag {
        DirectTag::to_direct(
            stream.index as u64,
            DirectInfo::ExtType(ExtType::Stream),
            DirectType::Ext,
        )
    }
}

pub trait Core {
    fn to_stream_index(_: &Mu, _: Tag) -> exception::Result<usize>;
    fn close(_: &Mu, _: Tag);
    fn get_string(_: &Mu, _: Tag) -> exception::Result<String>;
    fn is_open(_: &Mu, _: Tag) -> bool;
    fn read_byte(_: &Mu, _: Tag) -> exception::Result<Option<u8>>;
    fn read_char(_: &Mu, _: Tag) -> exception::Result<Option<char>>;
    fn unread_char(_: &Mu, _: Tag, _: char) -> exception::Result<Option<()>>;
    fn view(_: &Mu, _: Tag) -> Tag;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn write_byte(_: &Mu, _: Tag, _: u8) -> exception::Result<Option<()>>;
    fn write_char(_: &Mu, _: Tag, _: char) -> exception::Result<Option<()>>;
}

impl Core for Stream {
    fn to_stream_index(mu: &Mu, tag: Tag) -> exception::Result<usize> {
        match tag {
            Tag::Direct(dtag) => match dtag.dtype() {
                DirectType::Ext => match dtag.info() {
                    DirectTag::EXT_TYPE_STREAM => Ok(dtag.data() as usize),
                    _ => panic!(),
                },
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn view(mu: &Mu, tag: Tag) -> Tag {
        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();
                let vec = vec![
                    Tag::from(stream.index as i64),
                    stream.direction,
                    stream.unch,
                ];

                TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu)
            }
            None => panic!(),
        }
    }

    fn is_open(mu: &Mu, tag: Tag) -> bool {
        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();

                stream.open
            }
            None => panic!(),
        }
    }

    fn close(mu: &Mu, tag: Tag) {
        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();

                SystemStream::close(&stream.system);
            }
            None => panic!(),
        }
    }

    fn get_string(mu: &Mu, tag: Tag) -> exception::Result<String> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "get-str", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();

                Ok(SystemStream::get_string(&stream.system).unwrap())
            }
            None => panic!(),
        }
    }

    fn write(mu: &Mu, tag: Tag, _: bool, stream_tag: Tag) -> exception::Result<()> {
        match tag.type_of() {
            Type::Stream => {
                let streams_ref = block_on(mu.streams.read());

                match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
                    Some(stream_ref) => {
                        let stream = stream_ref.borrow();

                        mu.write_string(
                            format!(
                                "#<stream: id: {} type: {} dir: {} state: {}>",
                                stream.index,
                                match stream.system {
                                    SystemStream::File(_) => ":file",
                                    SystemStream::String(_) => ":string",
                                    SystemStream::StdInput => "std-in",
                                    SystemStream::StdOutput => "std-out",
                                    SystemStream::StdError => "err-out",
                                },
                                if Symbol::keyword("input").eq_(&stream.direction) {
                                    ":input"
                                } else if Symbol::keyword("output").eq_(&stream.direction) {
                                    ":output"
                                } else if Symbol::keyword("bidir").eq_(&stream.direction) {
                                    ":bidir"
                                } else {
                                    panic!()
                                },
                                if Self::is_open(mu, tag) {
                                    "open"
                                } else {
                                    "close"
                                },
                            )
                            .as_str(),
                            stream_tag,
                        )
                    }
                    None => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    fn read_char(mu: &Mu, tag: Tag) -> exception::Result<Option<char>> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "rd-char", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = stream_ref.borrow_mut();

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    return Err(Exception::new(Condition::Stream, "rd-char", tag));
                }

                if stream.unch.null_() {
                    match SystemStream::read_byte(&stream.system) {
                        Ok(opt) => match opt {
                            Some(byte) => Ok(Some(byte as char)),
                            None => Ok(None),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let unch = stream.unch;

                    stream.unch = Tag::nil();
                    Ok(Some(Char::as_char(mu, unch)))
                }
            }
            None => panic!(),
        }
    }

    fn read_byte(mu: &Mu, tag: Tag) -> exception::Result<Option<u8>> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "rd-byte", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = stream_ref.borrow_mut();

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    return Err(Exception::new(Condition::Stream, "rd-byte", tag));
                }

                if stream.unch.null_() {
                    match SystemStream::read_byte(&stream.system) {
                        Ok(opt) => match opt {
                            Some(byte) => Ok(Some(byte)),
                            None => Ok(None),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    let unch = stream.unch;

                    stream.unch = Tag::nil();

                    Ok(Some(Char::as_char(mu, unch) as u8))
                }
            }
            None => panic!(),
        }
    }

    fn unread_char(mu: &Mu, tag: Tag, ch: char) -> exception::Result<Option<()>> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "un-char", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let mut stream = stream_ref.borrow_mut();

                SystemStream::close(&stream.system);

                if stream.direction.eq_(&Symbol::keyword("output")) {
                    return Err(Exception::new(Condition::Type, "un-char", tag));
                }

                if stream.unch.null_() {
                    stream.unch = Tag::from(ch);

                    Ok(None)
                } else {
                    Err(Exception::new(Condition::Stream, "un-char", stream.unch))
                }
            }
            None => panic!(),
        }
    }

    fn write_char(mu: &Mu, tag: Tag, ch: char) -> exception::Result<Option<()>> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "wr-char", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    return Err(Exception::new(Condition::Type, "wr-char", tag));
                }

                SystemStream::write_byte(&stream.system, ch as u8)
            }
            None => panic!(),
        }
    }

    fn write_byte(mu: &Mu, tag: Tag, byte: u8) -> exception::Result<Option<()>> {
        if !Self::is_open(mu, tag) {
            return Err(Exception::new(Condition::Open, "wr-byte", tag));
        }

        let streams_ref = block_on(mu.streams.read());

        match streams_ref.get(Self::to_stream_index(mu, tag).unwrap()) {
            Some(stream_ref) => {
                let stream = stream_ref.borrow();

                if stream.direction.eq_(&Symbol::keyword("input")) {
                    return Err(Exception::new(Condition::Type, "wr-byte", tag));
                }

                SystemStream::write_byte(&stream.system, byte)
            }
            None => panic!(),
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
