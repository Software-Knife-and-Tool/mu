//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env fixnum type
use crate::{
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
    types::{cons::Cons, symbol::Symbol, vector::Vector},
};

impl From<usize> for Tag {
    fn from(fx: usize) -> Tag {
        assert!(Fixnum::is_i56(i64::try_from(fx).unwrap()));

        DirectTag::to_tag(
            u64::try_from(i64::try_from(fx).unwrap() & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl From<u32> for Tag {
    fn from(fx: u32) -> Tag {
        DirectTag::to_tag(
            u64::try_from(i64::from(fx) & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl From<u16> for Tag {
    fn from(fx: u16) -> Tag {
        DirectTag::to_tag(
            u64::try_from(i64::from(fx) & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl From<u8> for Tag {
    fn from(fx: u8) -> Tag {
        DirectTag::to_tag(
            u64::try_from(i64::from(fx) & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl From<Tag> for i64 {
    fn from(tag: Tag) -> i64 {
        assert_eq!(tag.type_of(), Type::Fixnum);

        i64::try_from(tag.as_u64()).unwrap() >> 8
    }
}

impl From<Tag> for usize {
    fn from(tag: Tag) -> usize {
        assert_eq!(tag.type_of(), Type::Fixnum);

        let data: i64 = tag.into();
        assert!(data >= 0);

        usize::try_from(data).unwrap()
    }
}

impl From<Tag> for i32 {
    fn from(tag: Tag) -> i32 {
        assert_eq!(tag.type_of(), Type::Fixnum);

        let data: i64 = tag.into();

        i32::try_from(data).unwrap()
    }
}

impl From<Tag> for u8 {
    fn from(tag: Tag) -> u8 {
        assert_eq!(tag.type_of(), Type::Fixnum);

        let data: i64 = tag.into();

        u8::try_from(data).unwrap()
    }
}

pub struct Fixnum;

impl Fixnum {
    const MAX: i64 = 2_i64.pow(55) - 1;
    const MIN: i64 = -(2_i64.pow(55));

    // range checking
    pub fn is_i56(i56: i64) -> bool {
        (Self::MIN..=Self::MAX).contains(&i56)
    }

    // untag fixnum
    pub fn as_i64(tag: Tag) -> i64 {
        assert_eq!(tag.type_of(), Type::Fixnum);

        #[allow(clippy::cast_possible_wrap)]
        let i64_ = tag.as_u64() as i64;

        i64_ >> 8
    }

    pub fn with_or_panic(fx: usize) -> Tag {
        Self::with_u64_or_panic(u64::try_from(fx).unwrap())
    }

    pub fn with_u64_or_panic(fx: u64) -> Tag {
        let i64_ = i64::try_from(fx).unwrap();
        assert!(Fixnum::is_i56(i64_));

        DirectTag::to_tag(
            u64::try_from(i64_ & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }

    pub fn with_i64_or_panic(fx: i64) -> Tag {
        assert!(Fixnum::is_i56(fx));

        DirectTag::to_tag(
            u64::try_from(fx & (2_i64.pow(56) - 1)).unwrap(),
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }

    pub fn with_u64(env: &Env, fx: u64) -> exception::Result<Tag> {
        match i64::try_from(fx) {
            Err(_) => Err(Exception::err(env, Tag::nil(), Condition::Over, "fixnum")),
            Ok(i64_) => {
                if !Fixnum::is_i56(i64_) {
                    return Err(Exception::err(env, Tag::nil(), Condition::Over, "fixnum"));
                }

                Ok(DirectTag::to_tag(
                    u64::try_from(i64::try_from(fx).unwrap() & (2_i64.pow(56) - 1)).unwrap(),
                    DirectExt::ExtType(ExtType::Fixnum),
                    DirectType::Ext,
                ))
            }
        }
    }

    pub fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        StreamWriter::write_str(env, &Self::as_i64(tag).to_string(), stream)
    }

    pub fn view(env: &Env, fx: Tag) -> Tag {
        Vector::from(vec![fx]).with_heap(env)
    }
}

pub trait CoreFn {
    fn mu_ash(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxadd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxdiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxlt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_fxsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_logand(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_lognot(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn mu_logor(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFn for Fixnum {
    fn mu_ash(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:ash", &[Type::Fixnum, Type::Fixnum], fp)?;

        let value = Self::as_i64(fp.argv[0]);
        let shift = Self::as_i64(fp.argv[1]);

        let result = if shift < 0 {
            value >> shift.abs()
        } else {
            value << shift
        };

        if Self::is_i56(result) {
            fp.value = Self::with_i64_or_panic(result);
        } else {
            Err(Exception::err(
                env,
                Cons::cons(env, fp.argv[0], fp.argv[1]),
                Condition::Over,
                "mu:ash",
            ))?;
        }

        Ok(())
    }

    fn mu_fxadd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:add", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match Self::as_i64(fx0).checked_add(Self::as_i64(fx1)) {
            Some(sum) => {
                if Self::is_i56(sum) {
                    Self::with_i64_or_panic(sum)
                } else {
                    Err(Exception::err(env, fx0, Condition::Over, "mu:add"))?
                }
            }
            None => Err(Exception::err(env, fx1, Condition::Over, "mu:add"))?,
        };

        Ok(())
    }

    fn mu_fxsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:sub", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match Self::as_i64(fx0).checked_sub(Self::as_i64(fx1)) {
            Some(diff) => {
                if Self::is_i56(diff) {
                    Self::with_i64_or_panic(diff)
                } else {
                    Err(Exception::err(env, fx0, Condition::Over, "mu:sub"))?
                }
            }
            None => Err(Exception::err(env, fx1, Condition::Over, "mu:sub"))?,
        };

        Ok(())
    }

    fn mu_fxmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:mul", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match Self::as_i64(fx0).checked_mul(Self::as_i64(fx1)) {
            Some(prod) => {
                if Self::is_i56(prod) {
                    Self::with_i64_or_panic(prod)
                } else {
                    Err(Exception::err(env, fx1, Condition::Over, "mu:mul"))?
                }
            }
            None => Err(Exception::err(env, fx1, Condition::Over, "mu:mul"))?,
        };

        Ok(())
    }

    fn mu_fxdiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:div", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        if Self::as_i64(fx1) == 0 {
            Err(Exception::err(env, fx0, Condition::ZeroDivide, "mu:div"))?;
        }

        fp.value = match Self::as_i64(fx0).checked_div(Self::as_i64(fx1)) {
            Some(div) => {
                if Self::is_i56(div) {
                    Self::with_i64_or_panic(div)
                } else {
                    Err(Exception::err(env, fx1, Condition::Over, "mu:div"))?
                }
            }
            None => Err(Exception::err(env, fx1, Condition::Over, "mu:div"))?,
        };

        Ok(())
    }

    fn mu_fxlt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:less-than", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = if Self::as_i64(fx0) < Self::as_i64(fx1) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_logand(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:logand", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = Self::with_i64_or_panic(Self::as_i64(fx0) & Self::as_i64(fx1));

        Ok(())
    }

    fn mu_logor(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:logor", &[Type::Fixnum, Type::Fixnum], fp)?;

        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = Self::with_i64_or_panic(Self::as_i64(fx0) | Self::as_i64(fx1));

        Ok(())
    }

    fn mu_lognot(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.argv_check("mu:lognot", &[Type::Fixnum], fp)?;

        let fx = fp.argv[0];
        let mut val = Self::as_i64(fx);

        for nth_bit in 0..64 {
            let mask = 1 << nth_bit;

            if val & mask == 0 {
                val |= mask;
            } else {
                val &= !mask;
            }
        }

        fp.value = Self::with_i64_or_panic(val);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn as_tag() {
        assert_eq!(true, true)
    }
}
