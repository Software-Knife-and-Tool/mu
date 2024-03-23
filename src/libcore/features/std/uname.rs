//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! posix interface
use {
    crate::{
        core::{
            exception::{self /* Condition, Exception */},
            frame::Frame,
            mu::Mu,
            types::Tag,
        },
        system::System,
        types::{
            cons::{Cons, Core as _},
            // fixnum::Fixnum,
            struct_::{Core as _, Struct},
            symbol::{Core as _, Symbol},
            // vecimage::VecType,
            vector::{Core as _, Vector},
        },
    },
    // std::{cell::RefCell, time::SystemTime},
    rustix::system,
};

pub trait MuFunction {
    fn posix_uname(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn posix_sysinfo(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl MuFunction for System {
    fn posix_uname(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let utsname = system::uname();

        let uname = vec![Cons::vlist(
            mu,
            &[
                Cons::new(
                    Symbol::keyword("sysname"),
                    Vector::from_string(utsname.sysname().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("nodenam"),
                    Vector::from_string(utsname.nodename().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("release"),
                    Vector::from_string(utsname.release().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("version"),
                    Vector::from_string(utsname.version().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("machine"),
                    Vector::from_string(utsname.machine().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("sysname"),
                    Vector::from_string(utsname.sysname().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                /*
                Cons::new(
                    Symbol::keyword("domname"),
                    Vector::from_string(utsname.domainname().to_str().unwrap()).evict(mu),
                )
                .evict(mu),
                */
            ],
        )];

        fp.value = Struct::new(mu, "utsname", uname).evict(mu);

        Ok(())
    }

    fn posix_sysinfo(_mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        /*
        let sysinfo = system::sysinfo();

        let info_vec = vec![Cons::vlist(
            mu,
            &[
                Cons::new(Symbol::keyword("uptime"), Fixnum::as_tag(sysinfo.uptime)).evict(mu),
                Cons::new(
                    Symbol::keyword("loads"),
                    vec![
                        sysinfo.loads[0] as i64,
                        sysinfo.loads[1] as i64,
                        sysinfo.loads[2] as i64,
                    ]
                    .to_vector()
                    .evict(mu),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("totlram"),
                    Fixnum::as_tag(sysinfo.totalram as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("freeram"),
                    Fixnum::as_tag(sysinfo.freeram as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("shrdram"),
                    Fixnum::as_tag(sysinfo.sharedram as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("bufram"),
                    Fixnum::as_tag(sysinfo.bufferram as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("totswap"),
                    Fixnum::as_tag(sysinfo.totalswap as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("freswap"),
                    Fixnum::as_tag(sysinfo.freeswap as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("procs"),
                    Fixnum::as_tag(sysinfo.procs as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("tothigh"),
                    Fixnum::as_tag(sysinfo.totalhigh as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("frehigh"),
                    Fixnum::as_tag(sysinfo.freehigh as i64),
                )
                .evict(mu),
                Cons::new(
                    Symbol::keyword("memunit"),
                    Fixnum::as_tag(sysinfo.mem_unit as i64),
                )
                .evict(mu),
            ],
        )];

        fp.value = Struct::new(mu, "sysinfo", info_vec).evict(mu);
         */

        fp.value = Tag::nil();
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
