//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/listener
extern crate mu;

use mu::{Condition};

pub fn _listener(system: &System, _config: &str) {
    let mu = system.mu();

    let eval_string = system
        .eval(&"(mu:open :string :output \"\")".to_string())
        .unwrap();

    let eof_value = system.eval(&"(mu:symbol \"eof\")".to_string()).unwrap();

    loop {
        match mu.read(mu.std_in(), true, eof_value) {
            Ok(expr) => {
                if mu.eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match mu.compile(expr) {
                    Ok(form) => match mu.eval(form) {
                        Ok(eval) => {
                            mu.write(eval, true, eval_string).unwrap();
                            println!("{}", mu.get_string(eval_string).unwrap());
                        }
                        Err(e) => {
                            eprint!(
                                "eval exception raised by {}, {:?} condition on ",
                                system.write(e.source, true),
                                e.condition
                            );
                            mu.write(e.object, true, mu.err_out()).unwrap();
                            eprintln!()
                        }
                    },
                    Err(e) => {
                        eprint!(
                            "compile exception raised by {}, {:?} condition on ",
                            system.write(e.source, true),
                            e.condition
                        );
                        mu.write(e.object, true, mu.err_out()).unwrap();
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
                    mu.write(e.object, true, mu.err_out()).unwrap();
                    eprintln!()
                }
            }
        }
    }
}
