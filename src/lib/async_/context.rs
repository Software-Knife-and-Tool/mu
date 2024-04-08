//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu async/await
#![allow(unused_imports)]
use {
    crate::{
        core::{
            apply::Core as _,
            compile::Compile as _,
            direct::{DirectInfo, DirectTag, DirectType, ExtType},
            exception::{self, Condition, Exception},
            frame::Frame,
            lib::Core as _,
            mu::{Core as _, Mu},
            types::{Tag, Type},
        },
        streams::{read::Core as _, write::Core as _},
        types::{
            cons::Cons,
            fixnum::Fixnum,
            function::Function,
            struct_::Struct,
            symbol::{Core as _, Symbol, UNBOUND},
        },
    },
    futures::{executor::block_on, future::BoxFuture, FutureExt},
    futures_locks::RwLock,
    std::assert,
};

pub struct Context {
    pub func: Tag,
    pub args: Tag,
    pub context: <Context as Core>::Future,
}

pub trait Core {
    type Future;

    fn context(_: &Mu, _: Tag, _: Tag) -> exception::Result<Tag>;
    fn write(_: &Mu, _: Tag, _: bool, _: Tag) -> exception::Result<()>;
}

impl Core for Context {
    type Future = BoxFuture<'static, Result<Tag, Exception>>;

    fn context(mu: &Mu, func: Tag, args: Tag) -> exception::Result<Tag> {
        let async_id = match func.type_of() {
            Type::Function => match args.type_of() {
                Type::Cons | Type::Null => {
                    let mut map_ref = block_on(mu.async_index.write());
                    let mut async_id: u64 = map_ref.len() as u64;

                    let mut tag = DirectTag::to_direct(
                        async_id,
                        DirectInfo::ExtType(ExtType::AsyncId),
                        DirectType::Ext,
                    );

                    let future: <Context as Core>::Future = Box::pin(async {
                        // mu.apply(func, args)
                        Ok(Tag::nil())
                    });

                    loop {
                        match map_ref.get(&tag.as_u64()) {
                            Some(_) => {
                                async_id += 1;
                                tag = DirectTag::to_direct(
                                    async_id,
                                    DirectInfo::ExtType(ExtType::AsyncId),
                                    DirectType::Ext,
                                );
                                continue;
                            }
                            None => {
                                map_ref.insert(
                                    tag.as_u64(),
                                    Context {
                                        func,
                                        args,
                                        context: future,
                                    },
                                );
                                break;
                            }
                        }
                    }

                    tag
                }
                _ => return Err(Exception::new(Condition::Type, "async", args)),
            },
            _ => return Err(Exception::new(Condition::Type, "async", func)),
        };

        Ok(async_id)
    }

    fn write(mu: &Mu, tag: Tag, _: bool, stream: Tag) -> exception::Result<()> {
        mu.write_string(
            format!("#<:asyncid [id:{}]>", Tag::data(&tag, mu)).as_str(),
            stream,
        )
    }
}

pub trait LibFunction {
    fn lib_await(_: &Mu, _: &mut Frame) -> exception::Result<()>;
    fn lib_abort(_: &Mu, _: &mut Frame) -> exception::Result<()>;
}

impl LibFunction for Context {
    fn lib_await(mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        let async_id = fp.argv[0];

        fp.value = match mu.fp_argv_check("await", &[Type::Vector], fp) {
            Ok(_) => {
                let map_ref = block_on(mu.async_index.write());

                match map_ref.get(&async_id.as_u64()) {
                    Some(_future) => Tag::nil(), // async {
                    _ => return Err(Exception::new(Condition::Range, "await", async_id)),
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn lib_abort(_mu: &Mu, fp: &mut Frame) -> exception::Result<()> {
        fp.value = Tag::nil();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn event() {
        assert_eq!(2 + 2, 4);
    }
}
