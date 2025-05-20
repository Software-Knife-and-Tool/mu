//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/listener
use mu_runtime::{Condition, Env, Mu};

pub fn _listener(env: &Env, _config: &str) {
    let eval_string = Mu::eval_str(env, &"(mu:open :string :output \"\")".to_string()).unwrap();

    let eof_value = Mu::eval_str(env, &"(env:symbol \"eof\")".to_string()).unwrap();

    loop {
        match Mu::read(env, Mu::std_in(), true, eof_value) {
            Ok(expr) => {
                if Mu::eq(expr, eof_value) {
                    break;
                }

                #[allow(clippy::single_match)]
                match Mu::compile(env, expr) {
                    Ok(form) => match Mu::eval(env, form) {
                        Ok(eval) => {
                            Mu::write(env, eval, true, eval_string).unwrap();
                            println!(
                                "{}",
                                Mu::eval_str(env, &"(mu:get-string eval-string)".to_string())
                                    .unwrap()
                            )
                        }
                        Err(e) => {
                            Mu::write(env, e.object, true, eval_string).unwrap();
                            let object =
                                Mu::eval_str(env, &"(mu:get-string eval-string)".to_string())
                                    .unwrap();
                            Mu::write(env, e.source, true, eval_string).unwrap();
                            let source =
                                Mu::eval_str(env, &"(mu:get-string eval-string)".to_string())
                                    .unwrap();

                            eprintln!(
                                "eval exception raised by {}, {:?} condition on {}",
                                source, e.condition, object
                            );
                        }
                    },
                    Err(e) => {
                        Mu::write(env, e.object, true, eval_string).unwrap();
                        let object =
                            Mu::eval_str(env, &"(mu:get-string eval-string)".to_string()).unwrap();
                        Mu::write(env, e.source, true, eval_string).unwrap();
                        let source =
                            Mu::eval_str(env, &"(mu:get-string eval-string)".to_string()).unwrap();

                        eprintln!(
                            "eval exception raised by {}, {:?} condition on {}",
                            source, e.condition, object
                        );
                    }
                }
            }
            Err(e) => {
                if let Condition::Eof = e.condition {
                    std::process::exit(0);
                } else {
                    Mu::write(env, e.object, true, eval_string).unwrap();
                    let object =
                        Mu::eval_str(env, &"(mu:get-string eval-string)".to_string()).unwrap();
                    Mu::write(env, e.source, true, eval_string).unwrap();
                    let source =
                        Mu::eval_str(env, &"(mu:get-string eval-string)".to_string()).unwrap();

                    eprintln!(
                        "eval exception raised by {}, {:?} condition on {}",
                        source, e.condition, object
                    );
                }
            }
        }
    }
}
