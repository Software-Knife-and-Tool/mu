//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! quasiquote reader
use crate::{
    core::{
        env::Env,
        exception::{self, Condition, Exception},
        lib::Lib,
        reader::Core as _,
        types::{Tag, Type},
    },
    streams::read::Core as _,
    types::{
        cons::{Cons, Core as _},
        core_stream::{Core as _, Stream},
        namespace::Namespace,
        symbol::{Core as _, Symbol},
    },
};

pub struct QuasiReader {
    stream: Tag,
    cons: Tag,
    qappend: Tag,
}

enum QuasiExpr {
    Comma(Box<QuasiExpr>),   // comma form
    CommaAt(Box<QuasiExpr>), // comma-at form
    Form(Tag),               // form
    List(Vec<QuasiExpr>),    // list
    Quasi(Box<QuasiExpr>),   // quasi form
    Quote(Tag),              // quote this form
}

enum QuasiToken {
    List(Tag),
    Atom(Tag),
}

#[derive(Debug)]
enum QuasiSyntax {
    Atom,
    Comma,
    CommaAt,
    List,
    List_,
    Quasi,
}

impl QuasiReader {
    pub fn new(env: &Env, stream: Tag) -> Self {
        Self {
            stream,
            cons: Namespace::intern(env, env.core_ns, "cons".to_string(), Tag::nil()).unwrap(),
            qappend: Namespace::intern(env, env.core_ns, "append".to_string(), Tag::nil()).unwrap(),
        }
    }

    pub fn read(env: &Env, _: bool, stream: Tag, _: bool) -> exception::Result<Tag> {
        let parser = Self::new(env, stream);
        let vec = parser.parse(env)?;

        let expansion = Self::compile(&parser, env, vec).unwrap();

        /*
        println!();
        env.debug_vprintln("compiles to:", true, expansion);
         */

        Ok(expansion)
    }

    fn append(&self, env: &Env, cons: Tag) -> Tag {
        if cons.null_() {
            return cons;
        }

        Cons::vlist(
            env,
            &[
                self.qappend,
                Cons::car(env, cons),
                self.append(env, Cons::cdr(env, cons)),
            ],
        )
    }

    #[allow(clippy::only_used_in_recursion)]
    fn compile(&self, env: &Env, expr: QuasiExpr) -> exception::Result<Tag> {
        match expr {
            QuasiExpr::Comma(boxed_expr) => Self::compile(self, env, *boxed_expr),
            QuasiExpr::CommaAt(boxed_expr) => Self::compile(self, env, *boxed_expr),
            QuasiExpr::Form(tag) => Ok(tag),
            QuasiExpr::List(vec) => {
                if vec.is_empty() {
                    Ok(Cons::vlist(env, &[self.cons, Tag::nil(), Tag::nil()]))
                } else {
                    let list = vec
                        .into_iter()
                        .map(|expr| self.compile(env, expr).unwrap())
                        .collect::<Vec<Tag>>();

                    Ok(self.append(env, Cons::vlist(env, &list)))
                }
            }
            QuasiExpr::Quote(form) => Ok(Cons::vlist(env, &[Symbol::keyword("quote"), form])),
            QuasiExpr::Quasi(qexpr) => match *qexpr {
                QuasiExpr::Comma(_) => panic!(),
                QuasiExpr::CommaAt(_) => panic!(),
                QuasiExpr::Form(tag) => Ok(tag),
                QuasiExpr::List(_) => panic!(),
                QuasiExpr::Quasi(_) => panic!(),
                QuasiExpr::Quote(_) => self.compile(env, *qexpr),
            },
        }
    }

    /*
        fn print_annotated_tag(env: &Env, preface: &str, tag: Tag) {
            env.debug_vprintln(preface, true, tag);
        }

        fn print_expr(env: &Env, indent: i32, expr: &QuasiExpr) {
            for _ in 1..indent * 2 {
                print!(" ");
            }
            match expr {
                QuasiExpr::Comma(boxed_expr) => {
                    println!("QuasiExpr::Comma: ");
                    Self::print_expr(env, 0, boxed_expr);
                }
                QuasiExpr::CommaAt(boxed_expr) => {
                    println!("QuasiExpr::CommaAt: ");
                    Self::print_expr(env, 0, boxed_expr);
                }
                QuasiExpr::Quote(tag) => Self::print_annotated_tag(env, "QuasiExpr::Quote:", *tag),
                QuasiExpr::Form(tag) => Self::print_annotated_tag(env, "QuasiExpr::Form:", *tag),
                QuasiExpr::List(vec) => {
                    println!("QuasiExpr::List: {}", vec.len());
                    for expr in vec {
                        Self::print_expr(env, indent + 1, expr);
                    }
                }
                QuasiExpr::Quasi(expr) => {
                    print!("QuasiExpr::Quasi: ");
                    Self::print_expr(env, 16, expr)
                }
            }
    }
        */

    fn read_syntax(&self, env: &Env, stream: Tag) -> exception::Result<Option<QuasiSyntax>> {
        Lib::read_ws(env, stream).unwrap();

        match Stream::read_char(env, stream)? {
            None => Ok(None),
            Some(ch) => match ch {
                '(' => Ok(Some(QuasiSyntax::List)),
                ')' => Ok(Some(QuasiSyntax::List_)),
                ',' => match Stream::read_char(env, stream)? {
                    None => Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "core:read",
                        Symbol::keyword("eof"),
                    )),
                    Some(ch) => {
                        if ch == '@' {
                            Ok(Some(QuasiSyntax::CommaAt))
                        } else {
                            Stream::unread_char(env, stream, ch).unwrap();
                            Ok(Some(QuasiSyntax::Comma))
                        }
                    }
                },
                '`' => Ok(Some(QuasiSyntax::Quasi)),
                _ => {
                    Stream::unread_char(env, stream, ch).unwrap();
                    Ok(Some(QuasiSyntax::Atom))
                }
            },
        }
    }

    fn read_form(env: &Env, stream: Tag) -> exception::Result<QuasiToken> {
        let form = env.read_stream(stream, false, Tag::nil(), false)?;

        match form.type_of() {
            Type::Cons => Ok(QuasiToken::List(form)),
            _ => Ok(QuasiToken::Atom(form)),
        }
    }

    fn parse_list(&self, env: &Env) -> exception::Result<QuasiExpr> {
        let mut expansion: Vec<QuasiExpr> = vec![];

        loop {
            match Self::read_syntax(self, env, self.stream)? {
                None => {
                    return Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "core:read",
                        Symbol::keyword("eof"),
                    ))
                }
                Some(syntax) => match syntax {
                    QuasiSyntax::Comma => match Self::read_form(env, self.stream)? {
                        QuasiToken::Atom(tag) | QuasiToken::List(tag) => expansion.push(
                            QuasiExpr::Form(Cons::vlist(env, &[self.cons, tag, Tag::nil()])),
                        ),
                    },
                    QuasiSyntax::CommaAt => match Self::read_form(env, self.stream)? {
                        QuasiToken::Atom(tag) | QuasiToken::List(tag) => {
                            expansion.push(QuasiExpr::Form(tag))
                        }
                    },
                    QuasiSyntax::Atom => match Self::read_form(env, self.stream)? {
                        QuasiToken::Atom(tag) => {
                            expansion.push(QuasiExpr::Quote(Cons::vlist(env, &[tag])))
                        }
                        _ => panic!(),
                    },
                    QuasiSyntax::List_ => return Ok(QuasiExpr::List(expansion)),
                    QuasiSyntax::List => {
                        Stream::unread_char(env, self.stream, '(').unwrap();
                        match Self::read_form(env, self.stream)? {
                            QuasiToken::List(tag) | QuasiToken::Atom(tag) => {
                                expansion.push(QuasiExpr::Quote(Cons::vlist(env, &[tag])))
                            }
                        }
                    }
                    QuasiSyntax::Quasi => expansion.push(self.parse(env).unwrap()),
                },
            }
        }
    }

    fn parse(&self, env: &Env) -> exception::Result<QuasiExpr> {
        match Self::read_syntax(self, env, self.stream)? {
            None => Err(Exception::new(
                env,
                Condition::Syntax,
                "core:read",
                Symbol::keyword("eof"),
            )),
            Some(syntax) => match syntax {
                QuasiSyntax::Atom => match Self::read_form(env, self.stream)? {
                    QuasiToken::Atom(tag) | QuasiToken::List(tag) => Ok(QuasiExpr::Quote(tag)),
                },
                QuasiSyntax::Comma => match Self::read_form(env, self.stream)? {
                    QuasiToken::Atom(tag) | QuasiToken::List(tag) => Ok(QuasiExpr::Form(tag)),
                },
                QuasiSyntax::CommaAt => Err(Exception::new(
                    env,
                    Condition::Syntax,
                    "core:read",
                    Symbol::keyword(",@"),
                )),
                QuasiSyntax::List_ => Err(Exception::new(
                    env,
                    Condition::Syntax,
                    "core:read",
                    Symbol::keyword(")"),
                )),
                QuasiSyntax::List => {
                    let expr = self.parse_list(env)?;

                    match expr {
                        QuasiExpr::List(ref vec) => {
                            if vec.is_empty() {
                                Ok(QuasiExpr::Form(Tag::nil()))
                            } else {
                                Ok(expr)
                            }
                        }
                        _ => Ok(expr),
                    }
                }
                QuasiSyntax::Quasi => match Self::read_form(env, self.stream)? {
                    QuasiToken::Atom(tag) | QuasiToken::List(tag) => Ok(QuasiExpr::Quote(tag)),
                },
            },
        }
    }
}
