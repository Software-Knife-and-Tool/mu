#[allow(unused_imports)]
use {
    crate::bindgen::{BindGen, Config},
    std::{
        env,
        fs::{self, File, OpenOptions},
        io::{self, Error, ErrorKind, Read, Result, Write},
    },
};

trait SymbolTable {}

impl SymbolTable for BindGen<'_> {}

#[cfg(test)]
mod tests {}
