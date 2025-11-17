//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
use {
    crate::env_::Env_,
    mu::{Condition, Mu, Result},
};

pub fn repl(env_: &Env_) -> Result {
    let env = env_.env;
    let ns = env_.ns.clone();

    let eof_value = Mu::eval_str(env, "'%eof%")?;
    let flush_form = Mu::compile(env, Mu::read_str(env, "(mu:flush mu:*standard-output*)")?)?;

    let read_form = match ns.as_str() {
        "mu" => Mu::read_str(env, "(mu:compile (mu:read mu:*standard-input* () '%eof%))")?,
        _ => Mu::read_str(
            env,
            "(core:compile (core:read mu:*standard-input* () '%eof%))",
        )?,
    };

    let prompt = format!("{ns}> ");

    loop {
        Mu::write_str(env, prompt.as_str(), Mu::std_out())?;
        Mu::eval(env, flush_form)?;

        match Mu::eval(env, read_form) {
            Ok(expr) => {
                if Mu::eq(expr, eof_value) {
                    break Ok(eof_value);
                }

                #[allow(clippy::single_match)]
                match Mu::eval(env, expr) {
                    Ok(form) => {
                        Mu::write(env, form, true, Mu::std_out())?;
                        println!()
                    }
                    Err(e) => {
                        eprint!(
                            "exception raised by {}, {:?} condition on ",
                            Mu::write_to_string(env, e.source, true),
                            e.condition,
                        );
                        Mu::write(env, e.object, true, Mu::err_out())?;
                        eprintln!()
                    }
                }
            }
            Err(e) => {
                if let Condition::Eof = e.condition {
                    std::process::exit(0);
                } else {
                    eprint!(
                        "reader exception raised by {}, {:?} condition on ",
                        Mu::write_to_string(env, e.source, true),
                        e.condition
                    );
                    Mu::write(env, e.object, true, Mu::err_out())?;
                    eprintln!()
                }
            }
        }
    }
}
