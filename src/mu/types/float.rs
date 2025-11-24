//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// float type
use {
    crate::{
        core::{
            apply::Apply as _,
            direct::{DirectExt, DirectTag, DirectType, ExtType},
            env::Env,
            exception::{self, Condition, Exception},
            frame::Frame,
            tag::Tag,
            type_::Type,
        },
        streams::writer::StreamWriter,
        types::{symbol::Symbol, vector::Vector},
    },
    std::ops::{Add, Div, Mul, Sub},
};

pub struct Float;

impl From<f32> for Tag {
    fn from(fl: f32) -> Tag {
        let bytes = fl.to_le_bytes();
        DirectTag::to_tag(
            u64::from(u32::from_le_bytes(bytes)),
            DirectExt::ExtType(ExtType::Float),
            DirectType::Ext,
        )
    }
}

impl Float {
    pub fn as_f32(env: &Env, tag: Tag) -> f32 {
        assert_eq!(tag.type_of(), Type::Float);

        let data = tag.data(env).to_le_bytes();
        let mut fl = 0.0f32.to_le_bytes();

        for (dst, src) in fl.iter_mut().zip(data.iter()) {
            *dst = *src;
        }
        f32::from_le_bytes(fl)
    }

    pub fn view(env: &Env, fl: Tag) -> Tag {
        Vector::from(vec![fl]).with_heap(env)
    }

    pub fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        StreamWriter::write_str(
            env,
            format!("{:.4}", Self::as_f32(env, tag)).as_str(),
            stream,
        )
    }
}

pub trait CoreFn {
    fn mu_fladd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_flmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fllt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fldiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Float {
    fn mu_fladd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fl-add", &[Type::Float, Type::Float], fp)?;

        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        let sum = Self::as_f32(env, fl0).add(Self::as_f32(env, fl1));
        fp.value = if sum.is_nan() {
            Err(Exception::new(env, Condition::Over, "mu:fl-add", fl1))?
        } else {
            sum.into()
        };

        Ok(())
    }

    fn mu_flsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fl-sub", &[Type::Float, Type::Float], fp)?;

        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        let diff = Self::as_f32(env, fl0).sub(Self::as_f32(env, fl1));
        fp.value = if diff.is_nan() {
            Err(Exception::new(env, Condition::Under, "mu:fl-sub", fl1))?
        } else {
            diff.into()
        };

        Ok(())
    }

    fn mu_flmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fl-mul", &[Type::Float, Type::Float], fp)?;

        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        let prod = Self::as_f32(env, fl0).mul(Self::as_f32(env, fl1));

        fp.value = if prod.is_nan() {
            Err(Exception::new(env, Condition::Over, "mu:fl-mul", fl1))?
        } else {
            prod.into()
        };

        Ok(())
    }

    fn mu_fldiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fl-div", &[Type::Float, Type::Float], fp)?;

        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

        if Self::as_f32(env, fl1) == 0.0 {
            Err(Exception::new(env, Condition::ZeroDivide, "fl-div", fl1))?;
        }

        let div = Self::as_f32(env, fl0).div(Self::as_f32(env, fl1));
        fp.value = if div.is_nan() {
            Err(Exception::new(env, Condition::Under, "mu:fl-div", fl1))?
        } else {
            div.into()
        };

        Ok(())
    }

    fn mu_fllt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:fl-lt", &[Type::Float, Type::Float], fp)?;

        let fl0 = fp.argv[0];
        let fl1 = fp.argv[1];

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
    use crate::core::tag::Tag;

    #[test]
    fn float_test() {
        match <f32 as Into<Tag>>::into(1.0_f32) {
            _ => assert_eq!(true, true),
        }
    }
}
