//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env float type
use {
    crate::{
        mu::{
            apply::Apply as _,
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            types::{Tag, Type},
        },
        streams::write::Write as _,
        types::{symbol::Symbol, vector::Vector},
    },
    std::ops::{Add, Div, Mul, Sub},
};

#[derive(Copy, Clone)]
pub struct Float {}

impl From<f32> for Tag {
    fn from(fl: f32) -> Tag {
        let bytes = fl.to_le_bytes();
        DirectTag::to_tag(
            u32::from_le_bytes(bytes) as u64,
            DirectExt::ExtType(ExtType::Float),
            DirectType::Ext,
        )
    }
}

impl Float {
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

    pub fn view(env: &Env, fl: Tag) -> Tag {
        Vector::from(vec![fl]).evict(env)
    }

    pub fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        env.write_string(format!("{:.4}", Self::as_f32(env, tag)).as_str(), stream)
    }
}

pub trait CoreFunction {
    fn mu_fladd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fllt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fldiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Float {
    fn mu_fladd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("mu:fl-add", &[Type::Float, Type::Float], fp)?;

        let sum = Self::as_f32(env, fl0).add(Self::as_f32(env, fl1));
        if sum.is_nan() {
            return Err(Exception::new(env, Condition::Over, "mu:fl-add", fl1));
        } else {
            fp.value = sum.into()
        }

        Ok(())
    }

    fn mu_flsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("mu:fl-sub", &[Type::Float, Type::Float], fp)?;

        let diff = Self::as_f32(env, fl0).sub(Self::as_f32(env, fl1));
        if diff.is_nan() {
            return Err(Exception::new(env, Condition::Under, "mu:fl-sub", fl1));
        } else {
            fp.value = diff.into()
        }

        Ok(())
    }

    fn mu_flmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("mu:fl-mul", &[Type::Float, Type::Float], fp)?;

        let prod = Self::as_f32(env, fl0).mul(Self::as_f32(env, fl1));
        if prod.is_nan() {
            return Err(Exception::new(env, Condition::Over, "mu:fl-mul", fl1));
        } else {
            fp.value = prod.into()
        }

        Ok(())
    }

    fn mu_fldiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("mu:fl-div", &[Type::Float, Type::Float], fp)?;

        if Self::as_f32(env, fl1) == 0.0 {
            return Err(Exception::new(env, Condition::ZeroDivide, "fl-div", fl1));
        }

        let div = Self::as_f32(env, fl0).div(Self::as_f32(env, fl1));

        fp.value = if div.is_nan() {
            return Err(Exception::new(env, Condition::Under, "mu:fl-div", fl1));
        } else {
            div.into()
        };

        Ok(())
    }

    fn mu_fllt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        env.fp_argv_check("mu:fl-lt", &[Type::Float, Type::Float], fp)?;
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
    use crate::mu::types::Tag;

    #[test]
    fn as_tag() {
        match <f32 as Into<Tag>>::into(1.0_f32) {
            _ => assert_eq!(true, true),
        }
    }
}
