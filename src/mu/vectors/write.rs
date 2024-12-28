//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! vector write function
use crate::{
    core::{
        direct::{DirectTag, DirectType},
        env::Env,
        exception,
        types::{Tag, Type},
    },
    streams::write::Write as _,
    types::{fixnum::Fixnum, stream::Write as _, vector::Vector},
};

use std::str;

pub trait Write {
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Write for Vector {
    fn write(env: &Env, vector: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => match str::from_utf8(&vector.data(env).to_le_bytes()) {
                    Ok(s) => {
                        if escape {
                            env.write_string("\"", stream).unwrap()
                        }

                        for nth in 0..DirectTag::length(vector) {
                            env.write_char(stream, s.as_bytes()[nth] as char)?;
                        }

                        if escape {
                            env.write_string("\"", stream).unwrap()
                        }

                        Ok(())
                    }
                    Err(_) => panic!(),
                },
                DirectType::ByteVec => {
                    env.write_string("#(:byte", stream)?;

                    for tag in Vector::iter(env, vector) {
                        env.write_string(" ", stream)?;
                        env.write_stream(tag, false, stream)?;
                    }

                    env.write_string(")", stream)
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => match Self::type_of(env, vector) {
                Type::Char => {
                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    for ch in Vector::iter(env, vector) {
                        env.write_stream(ch, false, stream)?;
                    }

                    if escape {
                        env.write_string("\"", stream)?;
                    }

                    Ok(())
                }
                Type::Bit => {
                    env.write_string("#*", stream)?;

                    let _len = Vector::length(env, vector);
                    for bit in Vector::iter(env, vector) {
                        let digit = Fixnum::as_i64(bit);

                        env.write_string(if digit == 1 { "1" } else { "0" }, stream)?
                    }

                    Ok(())
                }
                _ => {
                    env.write_string("#(", stream)?;
                    env.write_stream(Self::to_image(env, vector).type_, true, stream)?;

                    for tag in Vector::iter(env, vector) {
                        env.write_string(" ", stream)?;
                        env.write_stream(tag, false, stream)?;
                    }

                    env.write_string(")", stream)
                }
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
