//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// vector writer
use {
    crate::{
        core_::{
            direct::{DirectTag, DirectType},
            env::Env,
            exception,
            tag::Tag,
            type_::Type,
            writer::Writer,
        },
        streams::writer::StreamWriter,
        types::{fixnum::Fixnum, vector::Vector},
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
                DirectType::String => match str::from_utf8(&vector.data(env).to_le_bytes()) {
                    Ok(s) => {
                        if escape {
                            StreamWriter::write_str(env, "\"", stream).unwrap()
                        }

                        for nth in 0..DirectTag::length(vector) {
                            StreamWriter::write_char(env, stream, s.as_bytes()[nth] as char)?;
                        }

                        if escape {
                            StreamWriter::write_str(env, "\"", stream).unwrap()
                        }

                        Ok(())
                    }
                    Err(_) => panic!(),
                },
                DirectType::ByteVec => {
                    StreamWriter::write_str(env, "#(:byte", stream)?;

                    for tag in Vector::iter(env, vector) {
                        StreamWriter::write_str(env, " ", stream)?;
                        env.write(tag, false, stream)?;
                    }

                    StreamWriter::write_str(env, ")", stream)
                }
                _ => panic!(),
            },
            Tag::Indirect(_) => match Self::type_of(env, vector) {
                Type::Char => {
                    if escape {
                        StreamWriter::write_str(env, "\"", stream)?;
                    }

                    for ch in Vector::iter(env, vector) {
                        env.write(ch, false, stream)?;
                    }

                    if escape {
                        StreamWriter::write_str(env, "\"", stream)?;
                    }

                    Ok(())
                }
                Type::Bit => {
                    StreamWriter::write_str(env, "#*", stream)?;

                    for bit in Vector::iter(env, vector) {
                        let digit = Fixnum::as_i64(bit);

                        StreamWriter::write_str(env, if digit == 1 { "1" } else { "0" }, stream)?
                    }

                    Ok(())
                }
                _ => {
                    StreamWriter::write_str(env, "#(", stream)?;
                    env.write(Self::to_image(env, vector).type_, true, stream)?;

                    for tag in Vector::iter(env, vector) {
                        StreamWriter::write_str(env, " ", stream)?;
                        env.write(tag, false, stream)?;
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
