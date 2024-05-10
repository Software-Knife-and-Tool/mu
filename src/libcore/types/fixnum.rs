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
    fn core_ash(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fxadd(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fxdiv(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fxlt(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fxmul(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_fxsub(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_logand(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_lognot(_: &Env, _: &mut Frame) -> exception::Result<()>;
    fn core_logor(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Fixnum {
    fn core_ash(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        fp.value = match env.fp_argv_check("core:ash", &[Type::Fixnum, Type::Fixnum], fp) {
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
                        env,
                        Condition::Over,
                        "core:ash",
                        Cons::new(fp.argv[0], fp.argv[1]).evict(env),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_fxadd(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:fx-add", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_add(Self::as_i64(fx1)) {
                Some(sum) => {
                    if Self::is_i56(sum) {
                        Self::as_tag(sum)
                    } else {
                        return Err(Exception::new(env, Condition::Over, "core:fx-add", fx0));
                    }
                }
                None => return Err(Exception::new(env, Condition::Over, "core:fx-add", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_fxsub(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:fx-sub", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_sub(Self::as_i64(fx1)) {
                Some(diff) => {
                    if Self::is_i56(diff) {
                        Self::as_tag(diff)
                    } else {
                        return Err(Exception::new(env, Condition::Over, "core:fx-sub", fx1));
                    }
                }
                None => return Err(Exception::new(env, Condition::Over, "core:fx-sub", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_fxmul(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:fx-mul", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => match Self::as_i64(fx0).checked_mul(Self::as_i64(fx1)) {
                Some(prod) => {
                    if Self::is_i56(prod) {
                        Self::as_tag(prod)
                    } else {
                        return Err(Exception::new(env, Condition::Over, "core:fx-mul", fx1));
                    }
                }
                None => return Err(Exception::new(env, Condition::Over, "core:fx-mul", fx1)),
            },
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_fxdiv(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:fx-div", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => {
                if Self::as_i64(fx1) == 0 {
                    return Err(Exception::new(
                        env,
                        Condition::ZeroDivide,
                        "core:fx-div",
                        fx0,
                    ));
                }

                match Self::as_i64(fx0).checked_div(Self::as_i64(fx1)) {
                    Some(div) => {
                        if Self::is_i56(div) {
                            Self::as_tag(div)
                        } else {
                            return Err(Exception::new(env, Condition::Over, "core:fx-div", fx1));
                        }
                    }
                    None => return Err(Exception::new(env, Condition::Over, "core:fx-div", fx1)),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_fxlt(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:fx-lt", &[Type::Fixnum, Type::Fixnum], fp) {
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

    fn core_logand(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:logand", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => Self::as_tag(Self::as_i64(fx0) & Self::as_i64(fx1)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_logor(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx0 = fp.argv[0];
        let fx1 = fp.argv[1];

        fp.value = match env.fp_argv_check("core:logor", &[Type::Fixnum, Type::Fixnum], fp) {
            Ok(_) => Self::as_tag(Self::as_i64(fx0) | Self::as_i64(fx1)),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn core_lognot(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let fx = fp.argv[0];

        fp.value = match env.fp_argv_check("core:lognot", &[Type::Fixnum], fp) {
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
