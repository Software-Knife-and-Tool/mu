//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env float type
use {
    crate::{
        core::{
            apply::Core as _,
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        streams::write::Core as _,
        types::{
            symbol::{Core as _, Symbol},
            vecimage::{TypedVec, VecType},
            vector::Core as _,
        },
    },
    std::ops::{Add, Div, Mul, Sub},
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Float {
    Direct(u64),
}

impl From<f32> for Tag {
    fn from(fl: f32) -> Tag {
        let bytes = fl.to_le_bytes();
        DirectTag::to_direct(
            u32::from_le_bytes(bytes) as u64,
            DirectInfo::ExtType(ExtType::Float),
            DirectType::Ext,
        )
    }
}

impl Float {
    pub fn as_tag(fl: f32) -> Tag {
        let bytes = fl.to_le_bytes();
        DirectTag::to_direct(
            u32::from_le_bytes(bytes) as u64,
            DirectInfo::ExtType(ExtType::Float),
            DirectType::Ext,
        )
    }

    pub fn as_f32(env: &Env, tag: Tag) -> f32 {
        match tag.type_of() {
            Type::Float => {
                let data = tag.data(env).to_le_bytes();
                let mut fl = 0.0f32.to_le_bytes();

                for (dst, src) in fl.iter_mut().zip(data.iter()) {
                    *dst = *src
                }
                f32::from_le_bytes(fl)
            }
            _ => panic!(),
        }
    }
}

pub trait Core {
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
    fn view(_: &Env, _: Tag) -> Tag;
}

impl Core for Float {
    fn view(env: &Env, fl: Tag) -> Tag {
        let vec = vec![fl];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        env.write_string(format!("{:.4}", Self::as_f32(env, tag)).as_str(), stream)
    }
}

pub trait LibFunction {
    fn lib_fladd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_flsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_flenvl(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_fllt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn lib_fldiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Float {
    fn lib_fladd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        fp.value = match env.fp_argv_check("fl-add", &[Type::Float, Type::Float], fp) {
            Ok(_) => {
                let sum = Self::as_f32(env, fl0).add(Self::as_f32(env, fl1));
                if sum.is_nan() {
                    return Err(Exception::new(Condition::Over, "fl-add", fl1));
                } else {
                    Self::as_tag(sum)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_flsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        fp.value = match env.fp_argv_check("fl-sub", &[Type::Float, Type::Float], fp) {
            Ok(_) => {
                let diff = Self::as_f32(env, fl0).sub(Self::as_f32(env, fl1));
                if diff.is_nan() {
                    return Err(Exception::new(Condition::Under, "fl-sub", fl1));
                } else {
                    Self::as_tag(diff)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_flenvl(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        fp.value = match env.fp_argv_check("fl-envl", &[Type::Float, Type::Float], fp) {
            Ok(_) => {
                let prod = Self::as_f32(env, fl0).mul(Self::as_f32(env, fl1));

                if prod.is_nan() {
                    return Err(Exception::new(Condition::Over, "fl-envl", fl1));
                } else {
                    Self::as_tag(prod)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fldiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        fp.value = match env.fp_argv_check("fl-div", &[Type::Float, Type::Float], fp) {
            Ok(_) => {
                if Self::as_f32(env, fl1) == 0.0 {
                    return Err(Exception::new(Condition::ZeroDivide, "fl-div", fl1));
                }

                let div = Self::as_f32(env, fl0).div(Self::as_f32(env, fl1));
                if div.is_nan() {
                    return Err(Exception::new(Condition::Under, "fl-div", fl1));
                } else {
                    Self::as_tag(div)
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fllt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        fp.value = match env.fp_argv_check("fl-lt", &[Type::Float, Type::Float], fp) {
            Ok(_) => {
                if Self::as_f32(env, fl0) < Self::as_f32(env, fl1) {
                    Symbol::keyword("t")
                } else {
                    Tag::nil()
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::Tag;

    #[test]
    fn as_tag() {
        match Tag::from(1.0) {
            _ => assert_eq!(true, true),
        }
    }
}
