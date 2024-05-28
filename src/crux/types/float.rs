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
            indirect_vector::{TypedVector, VecType},
            symbol::{Core as _, Symbol},
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

        TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }

    fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        env.write_string(format!("{:.4}", Self::as_f32(env, tag)).as_str(), stream)
    }
}

pub trait CoreFunction {
    fn crux_fladd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_flsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_flmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fllt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fldiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Float {
    fn crux_fladd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("crux:fl-add", &[Type::Float, Type::Float], fp)?;

        let sum = Self::as_f32(env, fl0).add(Self::as_f32(env, fl1));
        if sum.is_nan() {
            return Err(Exception::new(env, Condition::Over, "crux:fl-add", fl1));
        } else {
            fp.value = Self::as_tag(sum)
        }

        Ok(())
    }

    fn crux_flsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("crux:fl-sub", &[Type::Float, Type::Float], fp)?;

        let diff = Self::as_f32(env, fl0).sub(Self::as_f32(env, fl1));
        if diff.is_nan() {
            return Err(Exception::new(env, Condition::Under, "crux:fl-sub", fl1));
        } else {
            fp.value = Self::as_tag(diff)
        }

        Ok(())
    }

    fn crux_flmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("crux:fl-mul", &[Type::Float, Type::Float], fp)?;

        let prod = Self::as_f32(env, fl0).mul(Self::as_f32(env, fl1));
        if prod.is_nan() {
            return Err(Exception::new(env, Condition::Over, "crux:fl-mul", fl1));
        } else {
            fp.value = Self::as_tag(prod)
        }

        Ok(())
    }

    fn crux_fldiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("crux:fl-div", &[Type::Float, Type::Float], fp)?;

        if Self::as_f32(env, fl1) == 0.0 {
            return Err(Exception::new(env, Condition::ZeroDivide, "fl-div", fl1));
        }

        let div = Self::as_f32(env, fl0).div(Self::as_f32(env, fl1));

        fp.value = if div.is_nan() {
            return Err(Exception::new(env, Condition::Under, "crux:fl-div", fl1));
        } else {
            Self::as_tag(div)
        };

        Ok(())
    }

    fn crux_fllt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("crux:fl-lt", &[Type::Float, Type::Float], fp)?;
        fp.value = if Self::as_f32(env, fl0) < Self::as_f32(env, fl1) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
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