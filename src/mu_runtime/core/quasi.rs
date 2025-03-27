//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! quasiquote reader
use crate::{
    core::{
        compile::Compile,
        env::Env,
        exception::{self, Condition, Exception},
        namespace::Namespace,
        reader::Reader,
        types::Tag,
    },
    streams::read::Read,
    types::{cons::Cons, stream::Read as _, symbol::Symbol},
};
use std::fmt;

pub struct QuasiReader {
    stream: Tag,
    cons: Tag,
    qappend: Tag,
}

enum QuasiExpr {
    Basic(Tag),
    Comma(Tag),
    CommaAt(Tag),
    List(Vec<QuasiExpr>),
}

#[derive(Debug)]
enum QuasiSyntax {
    Atom,
    Comma,
    CommaAt,
    ListStart,
    ListEnd,
    Quasi,
}

impl fmt::Display for QuasiExpr {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.0)
        Ok(())
    }
}

/// roughly CLHS Section 2.4.6
impl QuasiReader {
    pub fn new(env: &Env, stream: Tag) -> Self {
        Self {
            stream,
            cons: Namespace::intern(env, env.mu_ns, "cons".into(), Tag::nil()).unwrap(),
            qappend: Namespace::intern(env, env.mu_ns, "append".into(), Tag::nil()).unwrap(),
        }
    }

    /*
        fn print_annotated_tag(env: &Env, preface: &str, tag: Tag) {
            print!("[{:?}] ", tag.type_of());
            env.eprintln(preface, true, tag);
        }

        fn print_expr(env: &Env, indent: usize, expr: &QuasiExpr) {
            print!("{}", String::from_iter(vec![' '; indent * 2]));

            match expr {
                QuasiExpr::Basic(tag) => env.println("QuasiExpr::Basic", false, *tag),
                QuasiExpr::Comma(tag) => env.println("QuasiExpr::Comma", false, *tag),
                QuasiExpr::CommaAt(tag) => env.println("QuasiExpr::CommaAt", false, *tag),
                QuasiExpr::List(vec) => {
                    println!("QuasiExpr::List: {}", vec.len());
                    for expr in vec {
                        Self::print_expr(env, indent + 1, expr);
                    }
                }
            }
    }
        */

    pub fn read(env: &Env, _: bool, stream: Tag, _: bool) -> exception::Result<Tag> {
        let quasi = Self::new(env, stream);
        let expr = quasi.parse(env)?;
        let form = quasi.compile(env, &expr, false)?;

        Ok(Cons::list(env, &[quasi.qappend, form]))
    }

    fn read_syntax(&self, env: &Env) -> exception::Result<Option<QuasiSyntax>> {
        env.read_ws(self.stream)?;

        match env.read_char(self.stream)? {
            None => Ok(None),
            Some(ch) => match ch {
                '(' => Ok(Some(QuasiSyntax::ListStart)),
                ')' => Ok(Some(QuasiSyntax::ListEnd)),
                ',' => match env.read_char(self.stream)? {
                    None => Err(Exception::new(
                        env,
                        Condition::Stream,
                        "mu:read",
                        Symbol::keyword("eof"),
                    )),
                    Some(ch) => {
                        if ch == '@' {
                            Ok(Some(QuasiSyntax::CommaAt))
                        } else {
                            env.unread_char(self.stream, ch).unwrap();
                            Ok(Some(QuasiSyntax::Comma))
                        }
                    }
                },
                '`' => Ok(Some(QuasiSyntax::Quasi)),
                _ => {
                    env.unread_char(self.stream, ch).unwrap();
                    Ok(Some(QuasiSyntax::Atom))
                }
            },
        }
    }

    fn read_form(&self, env: &Env) -> exception::Result<Tag> {
        let form = env.read_stream(self.stream, false, Tag::nil(), false)?;

        Ok(form)
    }

    fn append_args(&self, env: &Env, cons: Tag) -> Tag {
        if cons.null_() {
            return cons;
        }

        Cons::list(
            env,
            &[
                self.cons,
                Cons::car(env, cons),
                self.append_args(env, Cons::cdr(env, cons)),
            ],
        )
    }

    fn compile(&self, env: &Env, expr: &QuasiExpr, recur: bool) -> exception::Result<Tag> {
        match expr {
            QuasiExpr::Basic(tag) => Ok(env.quote(&Cons::cons(env, *tag, Tag::nil()))),
            QuasiExpr::Comma(tag) => Ok(Cons::list(env, &[self.cons, *tag, Tag::nil()])),
            QuasiExpr::CommaAt(tag) => Ok(*tag),
            QuasiExpr::List(ref vec) => {
                if vec.is_empty() {
                    Ok(Cons::list(env, &[self.cons, Tag::nil(), Tag::nil()]))
                } else {
                    let list = vec
                        .iter()
                        .map(|expr| self.compile(env, expr, true).unwrap())
                        .collect::<Vec<Tag>>();
                    if recur {
                        Ok(Cons::list(
                            env,
                            &[
                                self.cons,
                                Cons::list(
                                    env,
                                    &[self.qappend, self.append_args(env, Cons::list(env, &list))],
                                ),
                                Tag::nil(),
                            ],
                        ))
                    } else {
                        Ok(self.append_args(env, Cons::list(env, &list)))
                    }
                }
            }
        }
    }

    fn parse_list(&self, env: &Env) -> exception::Result<QuasiExpr> {
        let mut expansion: Vec<QuasiExpr> = vec![];

        loop {
            match self.read_syntax(env)? {
                None => {
                    return Err(Exception::new(
                        env,
                        Condition::Stream,
                        "mu:read",
                        Symbol::keyword("eof"),
                    ))
                }
                Some(syntax) => match syntax {
                    QuasiSyntax::Atom => expansion.push(QuasiExpr::Basic(self.read_form(env)?)),
                    QuasiSyntax::Comma => expansion.push(QuasiExpr::Comma(self.read_form(env)?)),
                    QuasiSyntax::CommaAt => {
                        expansion.push(QuasiExpr::CommaAt(self.read_form(env)?))
                    }
                    QuasiSyntax::ListEnd => return Ok(QuasiExpr::List(expansion)),
                    QuasiSyntax::ListStart => expansion.push(self.parse_list(env)?),
                    QuasiSyntax::Quasi => expansion.push(self.parse(env).unwrap()),
                },
            }
        }
    }

    fn parse(&self, env: &Env) -> exception::Result<QuasiExpr> {
        match self.read_syntax(env)? {
            None => Err(Exception::new(
                env,
                Condition::Stream,
                "mu:read",
                Symbol::keyword("eof"),
            )),
            Some(syntax) => match syntax {
                QuasiSyntax::Atom => Ok(QuasiExpr::Basic(self.read_form(env)?)),
                QuasiSyntax::Comma => Ok(QuasiExpr::Comma(self.read_form(env)?)),
                QuasiSyntax::CommaAt => Err(Exception::new(
                    env,
                    Condition::Quasi,
                    "mu:read",
                    Symbol::keyword(",@"),
                )),
                QuasiSyntax::ListEnd => Err(Exception::new(
                    env,
                    Condition::Quasi,
                    "mu:read",
                    Symbol::keyword(")"),
                )),
                QuasiSyntax::ListStart => {
                    let expr = self.parse_list(env)?;

                    match expr {
                        QuasiExpr::List(ref vec) => {
                            if vec.is_empty() {
                                Ok(QuasiExpr::Basic(Tag::nil()))
                            } else {
                                Ok(expr)
                            }
                        }
                        _ => Ok(expr),
                    }
                }
                QuasiSyntax::Quasi => Ok(QuasiExpr::Basic(self.read_form(env)?)),
            },
        }
    }
}
