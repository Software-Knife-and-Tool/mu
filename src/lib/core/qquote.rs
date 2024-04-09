//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! quasiquote reader
use crate::{
    core::{
        exception::{self, Condition, Exception},
        lib::Lib,
        mu::Mu,
        namespace::Namespace,
        reader::Core as _,
        types::{Tag, Type},
    },
    streams::read::Core as _,
    types::{
        cons::{Cons, Core as _},
        stream::{Core as _, Stream},
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
    pub fn new(mu: &Mu, stream: Tag) -> Self {
        Self {
            stream,
            cons: Namespace::intern_symbol(mu, mu.lib_ns, "cons".to_string(), Tag::nil()),
            qappend: Namespace::intern_symbol(mu, mu.lib_ns, "append".to_string(), Tag::nil()),
        }
    }

    pub fn read(mu: &Mu, _: bool, stream: Tag, _: bool) -> exception::Result<Tag> {
        let parser = Self::new(mu, stream);
        match parser.parse(mu) {
            Ok(vec) => {
                let expansion = Self::compile(&parser, mu, vec).unwrap();

                /*
                println!();
                mu.debug_vprintln("compiles to:", true, expansion);
                 */

                Ok(expansion)
            }
            Err(e) => Err(e),
        }
    }

    fn append(&self, mu: &Mu, cons: Tag) -> Tag {
        if cons.null_() {
            return cons;
        }

        Cons::vlist(
            mu,
            &[
                self.qappend,
                Cons::car(mu, cons),
                self.append(mu, Cons::cdr(mu, cons)),
            ],
        )
    }

    #[allow(clippy::only_used_in_recursion)]
    fn compile(&self, mu: &Mu, expr: QqExpr) -> exception::Result<Tag> {
        match expr {
            QqExpr::Comma(boxed_expr) => Self::compile(self, mu, *boxed_expr),
            QqExpr::CommaAt(boxed_expr) => Self::compile(self, mu, *boxed_expr),
            QqExpr::Form(tag) => Ok(tag),
            QqExpr::List(vec) => {
                if vec.is_empty() {
                    Ok(Cons::vlist(mu, &[self.cons, Tag::nil(), Tag::nil()]))
                } else {
                    let list = vec
                        .into_iter()
                        .map(|expr| self.compile(mu, expr).unwrap())
                        .collect::<Vec<Tag>>();

                    Ok(self.append(mu, Cons::vlist(mu, &list)))
                }
            }
            QqExpr::Quote(form) => Ok(Cons::vlist(mu, &[Symbol::keyword("quote"), form])),
            QqExpr::Quasi(qexpr) => match *qexpr {
                QqExpr::Comma(_) => panic!(),
                QqExpr::CommaAt(_) => panic!(),
                QqExpr::Form(tag) => Ok(tag),
                QqExpr::List(_) => panic!(),
                QqExpr::Quasi(_) => panic!(),
                QqExpr::Quote(_) => self.compile(mu, *qexpr),
            },
        }
    }

    /*
        fn print_annotated_tag(mu: &Mu, preface: &str, tag: Tag) {
            mu.debug_vprintln(preface, true, tag);
        }

        fn print_expr(mu: &Mu, indent: i32, expr: &QqExpr) {
            for _ in 1..indent * 2 {
                print!(" ");
            }
            match expr {
                QqExpr::Comma(boxed_expr) => {
                    println!("QqExpr::Comma: ");
                    Self::print_expr(mu, 0, boxed_expr);
                }
                QqExpr::CommaAt(boxed_expr) => {
                    println!("QqExpr::CommaAt: ");
                    Self::print_expr(mu, 0, boxed_expr);
                }
                QqExpr::Quote(tag) => Self::print_annotated_tag(mu, "QqExpr::Quote:", *tag),
                QqExpr::Form(tag) => Self::print_annotated_tag(mu, "QqExpr::Form:", *tag),
                QqExpr::List(vec) => {
                    println!("QqExpr::List: {}", vec.len());
                    for expr in vec {
                        Self::print_expr(mu, indent + 1, expr);
                    }
                }
                QqExpr::Quasi(expr) => {
                    print!("QqExpr::Quasi: ");
                    Self::print_expr(mu, 16, expr)
                }
            }
    }
        */

    fn read_syntax(&self, mu: &Mu, stream: Tag) -> exception::Result<Option<QqSyntax>> {
        Lib::read_ws(mu, stream).unwrap();
        match Stream::read_char(mu, stream) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(ch)) => match ch {
                '(' => Ok(Some(QqSyntax::List)),
                ')' => Ok(Some(QqSyntax::List_)),
                ',' => match Stream::read_char(mu, stream) {
                    Err(e) => Err(e),
                    Ok(None) => Err(Exception::new(
                        Condition::Syntax,
                        "qquote",
                        Symbol::keyword("eof"),
                    )),
                    Ok(Some(ch)) => {
                        if ch == '@' {
                            Ok(Some(QqSyntax::CommaAt))
                        } else {
                            Stream::unread_char(mu, stream, ch).unwrap();
                            Ok(Some(QqSyntax::Comma))
                        }
                    }
                },
                '`' => Ok(Some(QqSyntax::Quasi)),
                _ => {
                    Stream::unread_char(mu, stream, ch).unwrap();
                    Ok(Some(QqSyntax::Atom))
                }
            },
        }
    }

    fn read_form(mu: &Mu, stream: Tag) -> exception::Result<QqToken> {
        match mu.read_stream(stream, false, Tag::nil(), false) {
            Err(e) => Err(e),
            Ok(form) => match form.type_of() {
                Type::Cons => Ok(QqToken::List(form)),
                _ => Ok(QqToken::Atom(form)),
            },
        }
    }

    fn parse_list(&self, mu: &Mu) -> exception::Result<QqExpr> {
        let mut expansion: Vec<QqExpr> = vec![];

        loop {
            match Self::read_syntax(self, mu, self.stream) {
                Err(e) => return Err(e),
                Ok(None) => {
                    return Err(Exception::new(
                        Condition::Syntax,
                        "qquote",
                        Symbol::keyword("eof"),
                    ))
                }
                Ok(Some(syntax)) => match syntax {
                    QqSyntax::Comma => match Self::read_form(mu, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) | QqToken::List(tag) => expansion
                                .push(QqExpr::Form(Cons::vlist(mu, &[self.cons, tag, Tag::nil()]))),
                        },
                    },
                    QqSyntax::CommaAt => match Self::read_form(mu, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) | QqToken::List(tag) => {
                                expansion.push(QqExpr::Form(tag))
                            }
                        },
                    },
                    QqSyntax::Atom => match Self::read_form(mu, self.stream) {
                        Err(e) => return Err(e),
                        Ok(form) => match form {
                            QqToken::Atom(tag) => {
                                expansion.push(QqExpr::Quote(Cons::vlist(mu, &[tag])))
                            }
                            _ => panic!(),
                        },
                    },
                    QqSyntax::List_ => return Ok(QqExpr::List(expansion)),
                    QqSyntax::List => {
                        Stream::unread_char(mu, self.stream, '(').unwrap();
                        match Self::read_form(mu, self.stream) {
                            Err(e) => return Err(e),
                            Ok(expr) => match expr {
                                QqToken::List(tag) | QqToken::Atom(tag) => {
                                    expansion.push(QqExpr::Quote(Cons::vlist(mu, &[tag])))
                                }
                            },
                        }
                    }
                    QqSyntax::Quasi => expansion.push(self.parse(mu).unwrap()),
                },
            }
        }
    }

    fn parse(&self, mu: &Mu) -> exception::Result<QqExpr> {
        match Self::read_syntax(self, mu, self.stream) {
            Err(e) => Err(e),
            Ok(None) => Err(Exception::new(
                Condition::Syntax,
                "qquote",
                Symbol::keyword("eof"),
            )),
            Ok(Some(syntax)) => match syntax {
                QqSyntax::Atom => match Self::read_form(mu, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Quote(tag)),
                    },
                },
                QqSyntax::Comma => match Self::read_form(mu, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Form(tag)),
                    },
                },
                QqSyntax::CommaAt => Err(Exception::new(
                    Condition::Syntax,
                    "qquote",
                    Symbol::keyword(",@"),
                )),
                QqSyntax::List_ => Err(Exception::new(
                    Condition::Syntax,
                    "qquote",
                    Symbol::keyword(")"),
                )),
                QqSyntax::List => match self.parse_list(mu) {
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
                QqSyntax::Quasi => match Self::read_form(mu, self.stream) {
                    Err(e) => Err(e),
                    Ok(form) => match form {
                        QqToken::Atom(tag) | QqToken::List(tag) => Ok(QqExpr::Quote(tag)),
                    },
                },
            },
        }
    }
}
