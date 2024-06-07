use {
    crate::config::Config,
    json::{self, JsonValue},
    std::{
        fs::{self, File},
        io::{Error, ErrorKind, Write},
        result::Result,
    },
    syn::{self},
};

pub struct BindGen<'a> {
    pub bindmap: Option<JsonValue>,
    pub config: &'a Config,
    pub syntax: syn::File,
}

impl BindGen<'_> {
    fn parse(path: &str) -> Result<syn::File, Error> {
        let source = fs::read(path)?;

        match syn::parse_file(&String::from_utf8(source).unwrap()) {
            Ok(syntax) => Ok(syntax),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    pub fn with_config<'a>(config: &'a Config, src_path: &str) -> Result<BindGen<'a>, Error> {
        let bindmap = match config.bindmap_path() {
            Some(path) => {
                let bindmap = fs::read(&path)?;

                match json::parse(&String::from_utf8(bindmap).unwrap()) {
                    Ok(bindmap) => Some(bindmap),
                    Err(_) => return Err(Error::new(ErrorKind::Other, path.to_string())),
                }
            }
            None => None,
        };

        let syntax = Self::parse(src_path)?;

        Ok(BindGen {
            config,
            bindmap,
            syntax,
        })
    }

    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = File::create(path)?;

        out.write_all(format!("{:#?}", self.syntax).as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
