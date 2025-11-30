//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// vector writer
use {
    crate::{
        core::{
            direct::{DirectTag, DirectType},
            env::Env,
            exception,
            tag::Tag,
        },
        streams::writer::StreamWriter,
        types::{
            fixnum::Fixnum,
            vector::{Vector, VectorType},
        },
    },
    std::str,
};

pub trait Write {
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Write for Vector {
    fn write(env: &Env, vector: Tag, escape: bool, stream: Tag) -> exception::Result<()> {
        match vector {
            Tag::Direct(direct) => match direct.dtype() {
                DirectType::String => {
                    let bytes = vector.data(env).to_le_bytes();
                    let str = str::from_utf8(&bytes).unwrap();

                    if escape {
                        StreamWriter::write_str(env, "\"", stream).unwrap();
                    }

                    for nth in 0..DirectTag::length(vector) {
                        StreamWriter::write_char(env, stream, str.as_bytes()[nth] as char)?;
                    }

                    if escape {
                        StreamWriter::write_str(env, "\"", stream).unwrap();
                    }

                    Ok(())
                }
                DirectType::ByteVec => {
                    StreamWriter::write_str(env, "#(:byte", stream)?;

                    for tag in Vector::iter(env, vector) {
                        StreamWriter::write_str(env, " ", stream)?;
                        StreamWriter::write(env, tag, false, stream)?;
                    }

                    StreamWriter::write_str(env, ")", stream)
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => match Self::vec_type_of(env, vector) {
                VectorType::Char(_) => {
                    if escape {
                        StreamWriter::write_str(env, "\"", stream)?;
                    }

                    for ch in Vector::iter(env, vector) {
                        StreamWriter::write(env, ch, false, stream)?;
                    }

                    if escape {
                        StreamWriter::write_str(env, "\"", stream)?;
                    }

                    Ok(())
                }
                VectorType::Bit(_) => {
                    StreamWriter::write_str(env, "#*", stream)?;

                    for bit in Vector::iter(env, vector) {
                        let digit = Fixnum::as_i64(bit);

                        StreamWriter::write_str(env, if digit == 1 { "1" } else { "0" }, stream)?;
                    }

                    Ok(())
                }
                _ => {
                    StreamWriter::write_str(env, "#(", stream)?;
                    StreamWriter::write(env, Self::to_image(env, vector).type_, true, stream)?;

                    for tag in Vector::iter(env, vector) {
                        StreamWriter::write_str(env, " ", stream)?;
                        StreamWriter::write(env, tag, false, stream)?;
                    }

                    StreamWriter::write_str(env, ")", stream)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
