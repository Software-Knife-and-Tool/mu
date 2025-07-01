//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! profile interface
use crate::{
    features::feature::Feature,
    mu::{
        apply::Apply as _,
        core::CoreFnDef,
        env::Env,
        exception::{self, Condition, Exception},
        frame::Frame,
        types::{Tag, Type},
    },
    types::{cons::Cons, fixnum::Fixnum, symbol::Symbol, vector::Vector},
};
use std::collections::HashMap;
use {futures_lite::future::block_on, futures_locks::RwLock};

pub trait Prof {
    fn feature() -> Feature;
    fn prof_event(_: &Env, _: Tag) -> exception::Result<()>;
}

lazy_static! {
    pub static ref PROF_SYMBOLS: RwLock<HashMap<String, Tag>> = RwLock::new(HashMap::new());
    pub static ref PROF_FUNCTIONS: Vec<CoreFnDef> =
        vec![("prof-control", 1, Feature::prof_control)];
}

impl Prof for Feature {
    fn feature() -> Feature {
        Feature {
            functions: Some(&PROF_FUNCTIONS),
            symbols: Some(&PROF_SYMBOLS),
            namespace: "prof".into(),
        }
    }

    fn prof_event(env: &Env, func: Tag) -> exception::Result<()> {
        if !*block_on(env.prof_on.read()) {
            return Ok(());
        }

        let mut profile_map_ref = block_on(env.prof.write());

        match (*profile_map_ref).iter().position(|item| func.eq_(&item.0)) {
            Some(index) => {
                let (func, count) = profile_map_ref[index];
                profile_map_ref[index] = (func, count + 1)
            }
            None => profile_map_ref.push((func, 1)),
        }

        Ok(())
    }
}

pub trait CoreFunction {
    fn prof_control(_: &Env, _: &mut Frame) -> exception::Result<()>;
}

impl CoreFunction for Feature {
    fn prof_control(env: &Env, fp: &mut Frame) -> exception::Result<()> {
        let cmd = fp.argv[0];

        env.fp_argv_check("prof:prof_control", &[Type::Keyword], fp)?;

        let profile_map_ref = block_on(env.prof.read());
        let mut prof_ref = block_on(env.prof_on.write());

        fp.value = cmd;
        if cmd.eq_(&Symbol::keyword("on")) {
            *prof_ref = true
        } else if cmd.eq_(&Symbol::keyword("off")) {
            *prof_ref = false
        } else if cmd.eq_(&Symbol::keyword("get")) {
            let prof_vec = (*profile_map_ref)
                .iter()
                .map(|item| Cons::cons(env, item.0, Fixnum::with_u64(env, item.1).unwrap()))
                .collect::<Vec<Tag>>();

            fp.value = Vector::from(prof_vec).evict(env)
        } else {
            return Err(Exception::new(
                env,
                Condition::Range,
                "profile:prof_control",
                cmd,
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
