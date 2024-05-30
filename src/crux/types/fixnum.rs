//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env fixnum type
use crate::{
    core::{
        apply::Core as _,
        direct::{DirectInfo, DirectTag, DirectType, ExtType},
        env::Env,
        exception::{self, Condition, Exception, Result},
        frame::Frame,
        types::{Tag, Type},
    },
    streams::write::Core as _,
    types::{
        cons::{Cons, Core as _},
        indirect_vector::{TypedVector, VecType},
        symbol::{Core as _, Symbol},
        vector::Core as _,
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Fixnum {
    Direct(u64),
}

// tag from i64
impl From<i64> for Tag {
    fn from(fx: i64) -> Tag {
        if !Fixnum::is_i56(fx) {
            panic!("fixnum overflow")
        }

        DirectTag::to_direct(
            (fx & (2_i64.pow(56) - 1)) as u64,
            DirectInfo::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

// tag from usize
impl From<usize> for Tag {
    fn from(fx: usize) -> Tag {
        (fx as i64).into()
    }
}

// tag from u64
impl From<u64> for Tag {
    fn from(fx: u64) -> Tag {
        (fx as i64).into()
    }
}

// tag from u32
impl From<u32> for Tag {
    fn from(fx: u32) -> Tag {
        (fx as i64).into()
    }
}

// tag from u16
impl From<u16> for Tag {
    fn from(fx: u16) -> Tag {
        (fx as i64).into()
    }
}

// tag from u8
impl From<u8> for Tag {
    fn from(fx: u8) -> Tag {
        (fx as i64).into()
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
    fn write(_: &Env, _: Tag, _: bool, _: Tag) -> Result<()>;
    fn view(_: &Env, _: Tag) -> Tag;
}

impl Core for Fixnum {
    fn write(env: &Env, tag: Tag, _escape: bool, stream: Tag) -> Result<()> {
        env.write_string(&Self::as_i64(tag).to_string(), stream)
    }

    fn view(env: &Env, fx: Tag) -> Tag {
        let vec = vec![fx];

        TypedVector::<Vec<Tag>> { vec }.vec.to_vector().evict(env)
    }
}

pub trait CoreFunction {
    fn crux_ash(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fxadd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fxdiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fxlt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fxmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_fxsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_logand(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_lognot(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn crux_logor(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Fixnum {
    fn crux_ash(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        env.fp_argv_check("crux:ash", &[Type::Fixnum, Type::Fixnum], fp)?;

        let value = Self::as_i64(fp.argv[0]);
        let shift = Self::as_i64(fp.argv[1]);

        let result = if shift < 0 {
            value >> shift.abs()
        } else {
            value << shift
        };

        if Self::is_i56(result) {
            fp.value = result.into()
        } else {
            return Err(Exception::new(
                env,
                Condition::Over,
                "crux:ash",
                Cons::new(fp.argv[0], fp.argv[1]).evict(env),
            ));
        }

        Ok(())
    }

    fn crux_fxadd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:sum", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_add(Self::as_i64(fx1)) {
            Some(sum) => {
                if Self::is_i56(sum) {
                    sum.into()
                } else {
                    return Err(Exception::new(env, Condition::Over, "crux:sum", fx0));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "crux:sum", fx1)),
        };

        Ok(())
    }

    fn crux_fxsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:difference", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_sub(Self::as_i64(fx1)) {
            Some(diff) => {
                if Self::is_i56(diff) {
                    diff.into()
                } else {
                    return Err(Exception::new(env, Condition::Over, "crux:difference", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "crux:difference", fx1)),
        };

        Ok(())
    }

    fn crux_fxmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:product", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = match Self::as_i64(fx0).checked_mul(Self::as_i64(fx1)) {
            Some(prod) => {
                if Self::is_i56(prod) {
                    prod.into()
                } else {
                    return Err(Exception::new(env, Condition::Over, "crux:product", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "crux:product", fx1)),
        };

        Ok(())
    }

    fn crux_fxdiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:quotient", &[Type::Fixnum, Type::Fixnum], fp)?;

        if Self::as_i64(fx1) == 0 {
            return Err(Exception::new(
                env,
                Condition::ZeroDivide,
                "crux:fx-div",
                fx0,
            ));
        }

        fp.value = match Self::as_i64(fx0).checked_div(Self::as_i64(fx1)) {
            Some(div) => {
                if Self::is_i56(div) {
                    div.into()
                } else {
                    return Err(Exception::new(env, Condition::Over, "crux:quotient", fx1));
                }
            }
            None => return Err(Exception::new(env, Condition::Over, "crux:quotient", fx1)),
        };

        Ok(())
    }

    fn crux_fxlt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:less-than", &[Type::Fixnum, Type::Fixnum], fp)?;

        fp.value = if Self::as_i64(fx0) < Self::as_i64(fx1) {
            Symbol::keyword("t")
        } else {
            Tag::nil()
        };

        Ok(())
    }

    fn crux_logand(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:logand", &[Type::Fixnum, Type::Fixnum], fp)?;
        fp.value = (Self::as_i64(fx0) & Self::as_i64(fx1)).into();

        Ok(())
    }

    fn crux_logor(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        env.fp_argv_check("crux:logor", &[Type::Fixnum, Type::Fixnum], fp)?;
        fp.value = (Self::as_i64(fx0) | Self::as_i64(fx1)).into();

        Ok(())
    }

    fn crux_lognot(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx = fp.argv[0];

        env.fp_argv_check("crux:lognot", &[Type::Fixnum], fp)?;

        let mut val = Self::as_i64(fx);
        for nth_bit in 0..64 {
            let mask = 1 << nth_bit;

            if val & mask == 0 {
                val |= mask
            } else {
                val &= !mask
            }
        }

        fp.value = val.into();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::Tag;

    #[test]
    fn as_tag() {
        match <i64 as Into<Tag>>::into(0_i64) {
            _ => assert_eq!(true, true),
        }
    }
}
