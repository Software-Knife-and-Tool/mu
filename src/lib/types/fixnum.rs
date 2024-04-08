//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu fixnum type
use crate::{
    core::{
        apply::Core as _,
        direct::{DirectInfo, DirectTag, DirectType, ExtType},
        exception::{self, Condition, Exception, Result},
        frame::Frame,
        mu::Mu,
        types::{Tag, Type},
    },
    streams::write::Core as _,
    types::{
        cons::{Cons, Core as _},
        symbol::{Core as _, Symbol},
        vecimage::{TypedVec, VecType},
        vector::Core as _,
    },
};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Fixnum {
    Direct(u64),
}

impl From<i64> for Tag {
    fn from(fx: i64) -> Tag {
        if !Fixnum::is_i56(fx) {
            panic!("fixnum overflow")
        }

        DirectTag::to_direct(
            (fx & (2i64.pow(56) - 1)) as u64,
            DirectInfo::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
    }
}

impl Fixnum {
    pub const FIXNUM_MAX: i64 = 2i64.pow(55) - 1;
    pub const FIXNUM_MIN: i64 = -(2i64.pow(55));

    // range checking
    pub fn is_i56(i56: i64) -> bool {
        (Self::FIXNUM_MIN..=Self::FIXNUM_MAX).contains(&i56)
    }

    // tag i64
    pub fn as_tag(fx: i64) -> Tag {
        if !Self::is_i56(fx) {
            panic!("fixnum overflow")
        }

        DirectTag::to_direct(
            (fx & (2i64.pow(56) - 1)) as u64,
            DirectInfo::ExtType(ExtType::Fixnum),
            DirectType::Ext,
        )
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
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> Result<()>;
    fn view(_: &Mu, _: Tag) -> Tag;
}

impl Core for Fixnum {
    fn write(mu: &Mu, tag: Tag, _escape: bool, stream: Tag) -> Result<()> {
        mu.write_string(&Self::as_i64(tag).to_string(), stream)
    }

    fn view(mu: &Mu, fx: Tag) -> Tag {
        let vec = vec![fx];

        TypedVec::<Vec<Tag>> { vec }.vec.to_vector().evict(mu)
    }
}

pub trait MuFunction {
    fn lib_ash(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fxadd(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fxdiv(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fxlt(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fxmul(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_fxsub(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_logand(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_lognot(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_logor(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for Fixnum {
    fn lib_ash(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match mu.fp_argv_check("ash", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => {
                let value = Self::as_i64(fp.argv[0]);
                let shift = Self::as_i64(fp.argv[1]);

                let result = if shift < 0 {
                    value >> shift.abs()
                } else {
                    value << shift
                };

                if Self::is_i56(result) {
                    Self::as_tag(result)
                } else {
                    return Err(Exception::new(
                        Condition::Over,
                        "ash",
                        Cons::new(fp.argv[0], fp.argv[1]).evict(mu),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fxadd(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("fx-add", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_add(Self::as_i64(fx1)) {
                Some(sum) => {
                    if Self::is_i56(sum) {
                        Self::as_tag(sum)
                    } else {
                        return Err(Exception::new(Condition::Over, "fx-add", fx0));
                    }
                }
                None => return Err(Exception::new(Condition::Over, "fx-add", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fxsub(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("fx-sub", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_sub(Self::as_i64(fx1)) {
                Some(diff) => {
                    if Self::is_i56(diff) {
                        Self::as_tag(diff)
                    } else {
                        return Err(Exception::new(Condition::Over, "fx-sub", fx1));
                    }
                }
                None => return Err(Exception::new(Condition::Over, "fx-sub", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fxmul(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("fx-mul", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_mul(Self::as_i64(fx1)) {
                Some(prod) => {
                    if Self::is_i56(prod) {
                        Self::as_tag(prod)
                    } else {
                        return Err(Exception::new(Condition::Over, "fx-mul", fx1));
                    }
                }
                None => return Err(Exception::new(Condition::Over, "fx-mul", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fxdiv(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("fx-div", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => {
                if Self::as_i64(fx1) == 0 {
                    return Err(Exception::new(Condition::ZeroDivide, "fx-div", fx0));
                }

                match Self::as_i64(fx0).checked_div(Self::as_i64(fx1)) {
                    Some(div) => {
                        if Self::is_i56(div) {
                            Self::as_tag(div)
                        } else {
                            return Err(Exception::new(Condition::Over, "fx-div", fx1));
                        }
                    }
                    None => return Err(Exception::new(Condition::Over, "fx-div", fx1)),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_fxlt(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("fx-lt", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => {
                if Self::as_i64(fx0) < Self::as_i64(fx1) {
                    Symbol::keyword("t")
                } else {
                    Tag::nil()
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_logand(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("logand", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => Self::as_tag(Self::as_i64(fx0) & Self::as_i64(fx1)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_logor(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match mu.fp_argv_check("logor", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => Self::as_tag(Self::as_i64(fx0) | Self::as_i64(fx1)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_lognot(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let fx = fp.argv[0];

        fp.value = match mu.fp_argv_check("lognot", &[Type::Fixnum], fp) {
            Ok(_) => {
                let mut val = Self::as_i64(fx);
                for nth_bit in 0..64 {
                    let mask = 1 << nth_bit;
                    if val & mask == 0 {
                        val |= mask
                    } else {
                        val &= !mask
                    }
                }

                Self::as_tag(val)
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
        match Tag::from(0i64) {
            _ => assert_eq!(true, true),
        }
    }
}
