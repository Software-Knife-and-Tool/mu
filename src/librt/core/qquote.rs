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

pub struct QqReader {
    stream: Tag,
    cons: Tag,
    qappend: Tag,
}

enum QqExpr {
    Comma(Box<QqExpr>),   // comma form
    CommaAt(Box<QqExpr>), // comma-at form
    Form(Tag),            // form
    List(Vec<QqExpr>),    // list
    Quasi(Box<QqExpr>),   // quasi form
    Quote(Tag),           // quote this form
}

enum QqToken {
    List(Tag),
    Atom(Tag),
}

#[derive(Debug)]
enum QqSyntax {
    Atom,
    Comma,
    CommaAt,
    List,
    List_,
    Quasi,
}

impl QqReader {
    pub fn new(env: &Env, stream: Tag) -> Self {
        Self {
            stream,
            cons: Namespace::intern(env, env.lib_ns, "cons".to_string(), Tag::nil()).unwrap(),
            qappend: Namespace::intern(env, env.lib_ns, "append".to_string(), Tag::nil()).unwrap(),
        }
    }

    pub fn read(env: &Env, _: bool, stream: Tag, _: bool) -> exception::Result<Tag> {
        let parser = Self::new(env, stream);
        match parser.parse(env) {
            Ok(vec) => {
                let expansion = Self::compile(&parser, env, vec).unwrap();

                /*
                println!();
                env.debug_vprintln("compiles to:", true, expansion);
                 */

                Ok(expansion)
            }
            Err(e) => Err(e),
        }
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
    fn compile(&self, env: &Env, expr: QqExpr) -> exception::Result<Tag> {
        match expr {
            QqExpr::Comma(boxed_expr) => Self::compile(self, env, *boxed_expr),
            QqExpr::CommaAt(boxed_expr) => Self::compile(self, env, *boxed_expr),
            QqExpr::Form(tag) => Ok(tag),
            QqExpr::List(vec) => {
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
            QqExpr::Quote(form) => Ok(Cons::vlist(env, &[Symbol::keyword("quote"), form])),
            QqExpr::Quasi(qexpr) => match *qexpr {
                QqExpr::Comma(_) => panic!(),
                QqExpr::CommaAt(_) => panic!(),
                QqExpr::Form(tag) => Ok(tag),
                QqExpr::List(_) => panic!(),
                QqExpr::Quasi(_) => panic!(),
                QqExpr::Quote(_) => self.compile(env, *qexpr),
            },
        }
    }

    /*
        fn print_annotated_tag(env: &Env, preface: &str, tag: Tag) {
            env.debug_vprintln(preface, true, tag);
        }

        fn print_expr(env: &Env, indent: i32, expr: &QqExpr) {
            for _ in 1..indent * 2 {
                print!(" ");
            }
            match expr {
                QqExpr::Comma(boxed_expr) => {
                    println!("QqExpr::Comma: ");
                    Self::print_expr(env, 0, boxed_expr);
                }
                QqExpr::CommaAt(boxed_expr) => {
                    println!("QqExpr::CommaAt: ");
                    Self::print_expr(env, 0, boxed_expr);
                }
                QqExpr::Quote(tag) => Self::print_annotated_tag(env, "QqExpr::Quote:", *tag),
                QqExpr::Form(tag) => Self::print_annotated_tag(env, "QqExpr::Form:", *tag),
                QqExpr::List(vec) => {
                    println!("QqExpr::List: {}", vec.len());
                    for expr in vec {
                        Self::print_expr(env, indent + 1, expr);
                    }
                }
                QqExpr::Quasi(expr) => {
                    print!("QqExpr::Quasi: ");
                    Self::print_expr(env, 16, expr)
                }
            }
    }
        */

    fn read_syntax(&self, env: &Env, stream: Tag) -> exception::Result<Option<QqSyntax>> {
        Lib::read_ws(env, stream).unwrap();
        match Stream::read_char(env, stream) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(ch)) => match ch {
                '(' => Ok(Some(QqSyntax::List)),
                ')' => Ok(Some(QqSyntax::List_)),
                ',' => match Stream::read_char(env, stream) {
                    Err(e) => Err(e),
                    Ok(None) => Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "qquote",
                        Symbol::keyword("eof"),
                    )),
                    Ok(Some(ch)) => {
                        if ch == '@' {
                            Ok(Some(QqSyntax::CommaAt))
                        } else {
                            Stream::unread_char(env, stream, ch).unwrap();
                            Ok(Some(QqSyntax::Comma))
                        }
                    }
                },
                '`' => Ok(Some(QqSyntax::Quasi)),
                _ => {
                    Stream::unread_char(env, stream, ch).unwrap();
                    Ok(Some(QqSyntax::Atom))
                }
            },
        }
    }

    fn read_form(env: &Env, stream: Tag) -> exception::Result<QqToken> {
        match env.read_stream(stream, false, Tag::nil(), false) {
            Err(e) => Err(e),
            Ok(form) => match form.type_of() {
                Type::Cons => Ok(QqToken::List(form)),
                _ => Ok(QqToken::Atom(form)),
            },
        }
    }

    fn parse_list(&self, env: &Env) -> exception::Result<QqExpr> {
        let mut expansion: Vec<QqExpr> = vec![];

        loop {
            match Self::read_syntax(self, env, self.stream) {
                Err(e) => return Err(e),
                Ok(None) => {
                    return Err(Exception::new(
                        env,
                        Condition::Syntax,
                        "qquote",
                        Symbol::keyword("eof"),
                    ))
                }
                Ok(Some(syntax)) => match syntax {
                    QqSyntax::Comma => match Self::read_form(env, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) | QqToken::List(tag) => expansion.push(
                                QqExpr::Form(Cons::vlist(env, &[self.cons, tag, Tag::nil()])),
                            ),
                        },
                    },
                    QqSyntax::CommaAt => match Self::read_form(env, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) | QqToken::List(tag) => {
                                expansion.push(QqExpr::Form(tag))
                            }
                        },
                    },
                    QqSyntax::Atom => match Self::read_form(env, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) => {
                                expansion.push(QqExpr::Quote(Cons::vlist(env, &[tag])))
                            }
                            _ => panic!(),
                        },
                    },
                    QqSyntax::List_ => return Ok(QqExpr::List(expansion)),
                    QqSyntax::List => {
                        Stream::unread_char(env, self.stream, '(').unwrap();
                        match Self::read_form(env, self.stream) {
                            Err(e) => return Err(e),
                            Ok(expr) => match expr {
                                QqToken::List(tag) | QqToken::Atom(tag) => {
                                    expansion.push(QqExpr::Quote(Cons::vlist(env, &[tag])))
                                }
                            },
                        }
                    }
                    QqSyntax::Quasi => expansion.push(self.parse(env).unwrap()),
                },
            }
        }
    }

    fn parse(&self, env: &Env) -> exception::Result<QqExpr> {
        match Self::read_syntax(self, env, self.stream) {
            Err(e) => Err(e),
            Ok(None) => Err(Exception::new(
                env,
                Condition::Syntax,
                "qquote",
                Symbol::keyword("eof"),
            )),
            Ok(Some(syntax)) => match syntax {
                QqSyntax::Atom => match Self::read_form(env, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Quote(tag)),
                    },
                },
                QqSyntax::Comma => match Self::read_form(env, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Form(tag)),
                    },
                },
                QqSyntax::CommaAt => Err(Exception::new(
                    env,
                    Condition::Syntax,
                    "qquote",
                    Symbol::keyword(",@"),
                )),
                QqSyntax::List_ => Err(Exception::new(
                    env,
                    Condition::Syntax,
                    "qquote",
                    Symbol::keyword(")"),
                )),
                QqSyntax::List => match self.parse_list(env) {
                    Err(e) => Err(e),
                    Ok(expr) => match expr {
                        QqExpr::List(ref vec) => {
                            if vec.is_empty() {
                                Ok(QqExpr::Form(Tag::nil()))
                            } else {
                                Ok(expr)
                            }
                        }
                        _ => Ok(expr),
                    },
                },
                QqSyntax::Quasi => match Self::read_form(env, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Quote(tag)),
                    },
                },
            },
        }
    }
}
