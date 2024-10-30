//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env fixnum type
use crate::{
    core::{
        apply::Core as _,
        direct::{DirectExt, DirectTag, DirectType, ExtType},
        env::Env,
        exception::{self, Condition, Exception, Result},
        frame::Frame,
        types::{Tag, Type},
    },
    streams::write::Core as _,
    types::{
        cons::{Cons, Core as _},
        symbol::{Core as _, Symbol},
        vector::Vector,
        vector_image::Core as _,
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Fixnum {
    Direct(u64),
}

// tag from u32
impl From<u32> for Tag {
    fn from(fx: u32) -> Tag {
        DirectTag::to_direct(
            ((fx as i64) & (2_i64.pow(56) - 1)) as u64,
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

// tag from u16
impl From<u16> for Tag {
    fn from(fx: u16) -> Tag {
        DirectTag::to_direct(
            ((fx as i64) & (2_i64.pow(56) - 1)) as u64,
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

// tag from u8
impl From<u8> for Tag {
    fn from(fx: u8) -> Tag {
        DirectTag::to_direct(
            ((fx as i64) & (2_i64.pow(56) - 1)) as u64,
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl Fixnum {
    pub const FIXNUM_MAX: i64 = 2_i64.pow(55) - 1;
    pub const FIXNUM_MIN: i64 = -(2_i64.pow(55));

    // range checking
    pub fn is_i56(i56: i64) -> bool {
        (Self::FIXNUM_MIN..=Self::FIXNUM_MAX).contains(&i56)
    }

    // untag fixnum
    pub fn as_i64(tag: Tag) -> i64 {
        match tag.type_of() {
            Type::Fixnum => (tag.as_u64() as i64) >> 8,
            _ => panic!(),
        }
    }
}

pub trait Core {
    fn with(_: &Env, _: usize) -> exception::Result<Tag>;
    fn with_or_panic(_: usize) -> Tag;
    fn with_i64(env: &Env, fx: i64) -> exception::Result<Tag>;
    fn with_i64_or_panic(_: i64) -> Tag;
    fn with_u64(_: &Env, _: u64) -> exception::Result<Tag>;
    fn with_u64_or_panic(_: u64) -> Tag;
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> Result<()>;
    fn view(_: &Env, _: Tag) -> Tag;
}

//
// assume that u64 and usize are the same size
// and an as cast works without losing information
//
impl Core for Fixnum {
    fn with_or_panic(fx: usize) -> Tag {
        Self::with_u64_or_panic(fx as u64)
    }

    fn with_u64_or_panic(fx: u64) -> Tag {
        match i64::try_from(fx) {
            Err(_) => panic!(),
            Ok(i64_) => {
                if !Fixnum::is_i56(i64_) {
                    panic!()
                }

                DirectTag::to_direct(
                    (i64_ & (2_i64.pow(56) - 1)) as u64,
                    DirectExt::ExtType(ExtType::Fixnum),
                    DirectType::Ext,
                )
            }
        }
    }

    fn with_i64_or_panic(fx: i64) -> Tag {
        if !Fixnum::is_i56(fx) {
            panic!()
        }

        DirectTag::to_direct(
            (fx & (2_i64.pow(56) - 1)) as u64,
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }

    fn with(env: &Env, fx: usize) -> exception::Result<Tag> {
        Self::with_u64(env, fx as u64)
    }

    fn with_i64(env: &Env, fx: i64) -> exception::Result<Tag> {
        if !Fixnum::is_i56(fx) {
            return Err(Exception::new(env, Condition::Over, "fixnum", Tag::nil()));
        }

        Ok(DirectTag::to_direct(
            (fx & (2_i64.pow(56) - 1)) as u64,
            DirectExt::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        ))
    }

    fn with_u64(env: &Env, fx: u64) -> exception::Result<Tag> {
        match i64::try_from(fx) {
            Err(_) => Err(Exception::new(env, Condition::Over, "fixnum", Tag::nil())),
            Ok(i64_) => {
                if !Fixnum::is_i56(i64_) {
                    return Err(Exception::new(env, Condition::Over, "fixnum", Tag::nil()));
                }

                Ok(DirectTag::to_direct(
                    ((fx as i64) & (2_i64.pow(56) - 1)) as u64,
                    DirectExt::ExtType(ExtType::Fixnum),
                    DirectType::Ext,
                ))
            }
        }
    }

    fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> exception::Result<()> {
        env.write_string(&Self::as_i64(tag).to_string(), stream)
    }

    fn view(env: &Env, fx: Tag) -> Tag {
        Vector::from(vec![fx]).evict(env)
    }
}

pub trait CoreFunction {
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

impl CoreFunction for Fixnum {
    fn mu_ash(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.fp_argv_check("mu:ash", &[Type::Fixnum, Type::Fixnum], fp)?;

        let value = Self::as_i64(fp.argv[0]);
        let shift = Self::as_i64(fp.argv[1]);

        let result = if shift < 0 {
            value >> shift.abs()
        } else {
            value << shift
        };

        if Self::is_i56(result) {
            fp.value = Self::with_i64_or_panic(result)
        } else {
            return Err(Exception::new(
                env,
                Condition::Over,
                "mu:ash",
                Cons::cons(env, fp.argv[0], fp.argv[1]),
            ));
        }

        Ok(())
    }

    fn mu_fxadd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:add", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_add(Self::as_i64(fx1)) {
            Some(sum) => {
                if Self::is_i56(sum) {
                    Self::with_i64_or_panic(sum)
                } else {
                    return Err(Exception::new(env, Condition::Over, "mu:add", fx0));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "mu:add", fx1)),
        };

        Ok(())
    }

    fn mu_fxsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:sub", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_sub(Self::as_i64(fx1)) {
            Some(diff) => {
                if Self::is_i56(diff) {
                    Self::with_i64_or_panic(diff)
                } else {
                    return Err(Exception::new(env, Condition::Over, "mu:sub", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "mu:sub", fx1)),
        };

        Ok(())
    }

    fn mu_fxmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:mul", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_mul(Self::as_i64(fx1)) {
            Some(prod) => {
                if Self::is_i56(prod) {
                    Self::with_i64_or_panic(prod)
                } else {
                    return Err(Exception::new(env, Condition::Over, "mu:mul", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "mu:mul", fx1)),
        };

        Ok(())
    }

    fn mu_fxdiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:div", &[Type::Fixnum, Type::Fixnum], fp)?;

        if Self::as_i64(fx1) == 0 {
            return Err(Exception::new(env, Condition::ZeroDivide, "mu:fx-div", fx0));
        }

        fp.value = match Self::as_i64(fx0).checked_div(Self::as_i64(fx1)) {
            Some(div) => {
                if Self::is_i56(div) {
                    Self::with_i64_or_panic(div)
                } else {
                    return Err(Exception::new(env, Condition::Over, "mu:div", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "mu:div", fx1)),
        };

        Ok(())
    }

    fn mu_fxlt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:less-than", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = if Self::as_i64(fx0) < Self::as_i64(fx1) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn mu_logand(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:logand", &[Type::Fixnum, Type::Fixnum], fp)?;
        fp.value = Self::with_i64_or_panic(Self::as_i64(fx0) & Self::as_i64(fx1));

        Ok(())
    }

    fn mu_logor(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("mu:logor", &[Type::Fixnum, Type::Fixnum], fp)?;
        fp.value = Self::with_i64_or_panic(Self::as_i64(fx0) | Self::as_i64(fx1));

        Ok(())
    }

    fn mu_lognot(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx = fp.argv[0];

        env.fp_argv_check("mu:lognot", &[Type::Fixnum], fp)?;

        let mut val = Self::as_i64(fx);
        for nth_bit in 0..64 {
            let mask = 1 << nth_bit;

            if val & mask == 0 {
                val |= mask
            } else {
                val &= !mask
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
