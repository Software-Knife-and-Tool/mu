//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/listener
use mu_runtime::{Condition, Env};

pub fn _listener(env: &Env, _config: &str) {
    let eval_string = env
        .eval_str(&"(mu:open :string :output \"\")".to_string())
        .unwrap();

    let eof_value = env.eval_str(&"(env:symbol \"eof\")".to_string()).unwrap();

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
                            println!(
                                "{}",
                                env.eval_str(&"(mu:get-string eval-string)".to_string())
                                    .unwrap()
                            )
                        }
                        Err(e) => {
                            env.write(e.object, true, eval_string).unwrap();
                            let object = env
                                .eval_str(&"(mu:get-string eval-string)".to_string())
                                .unwrap();
                            env.write(e.source, true, eval_string).unwrap();
                            let source = env
                                .eval_str(&"(mu:get-string eval-string)".to_string())
                                .unwrap();

                            eprintln!(
                                "eval exception raised by {}, {:?} condition on {}",
                                source, e.condition, object
                            );
                        }
                    },
                    Err(e) => {
                        env.write(e.object, true, eval_string).unwrap();
                        let object = env
                            .eval_str(&"(mu:get-string eval-string)".to_string())
                            .unwrap();
                        env.write(e.source, true, eval_string).unwrap();
                        let source = env
                            .eval_str(&"(mu:get-string eval-string)".to_string())
                            .unwrap();

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
                    env.write(e.object, true, eval_string).unwrap();
                    let object = env
                        .eval_str(&"(mu:get-string eval-string)".to_string())
                        .unwrap();
                    env.write(e.source, true, eval_string).unwrap();
                    let source = env
                        .eval_str(&"(mu:get-string eval-string)".to_string())
                        .unwrap();

                    eprintln!(
                        "eval exception raised by {}, {:?} condition on {}",
                        source, e.condition, object
                    );
                }
            }
        }
    }
}
