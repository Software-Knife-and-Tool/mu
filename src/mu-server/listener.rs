//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/listener
extern crate env;

use env::{Condition};

pub fn _listener(system: &System, _config: &str) {
    let env = system.env();

    let eval_string = system
        .eval(&"(env:open :string :output \"\")".to_string())
        .unwrap();

    let eof_value = system.eval(&"(env:symbol \"eof\")".to_string()).unwrap();

    loop {
        match env.read(env.std_in(), true, eof_value) {
            Ok(expr) => {
                if env.eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match env.compile(expr) {
                    Ok(form) => match env.eval(form) {
                        Ok(eval) => {
                            env.write(eval, true, eval_string).unwrap();
                            println!("{}", env.get_string(eval_string).unwrap());
                        }
                        Err(e) => {
                            eprint!(
                                "eval exception raised by {}, {:?} condition on ",
                                system.write(e.source, true),
                                e.condition
                            );
                            env.write(e.object, true, env.err_out()).unwrap();
                            eprintln!()
                        }
                    },
                    Err(e) => {
                        eprint!(
                            "compile exception raised by {}, {:?} condition on ",
                            system.write(e.source, true),
                            e.condition
                        );
                        env.write(e.object, true, env.err_out()).unwrap();
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
                        system.write(e.source, true),
                        e.condition
                    );
                    env.write(e.object, true, env.err_out()).unwrap();
                    eprintln!()
                }
            }
        }
    }
}
