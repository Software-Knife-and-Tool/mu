//  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! image mechanics
use {
    crate::{reader::Reader, writer::Writer},
    json::{self, object},
    mu_runtime::{Env, Mu},
};

#[allow(dead_code)]
pub struct Image {
    pub magic: String,
    pub version: String,
}

impl Image {
    pub const IMAGE_MAGIC: &'static str = "mu-image";

    pub fn load_image(path: &str) -> Option<Image> {
        let reader = Reader::with(path).unwrap();

        #[allow(unused_assignments)]
        let mut option = None;

        match reader.section_by_name(".ident") {
            Some(section) => {
                let image = reader.section_data(section).unwrap();
                let ident = json::parse(&String::from_utf8(image.to_vec()).unwrap()).unwrap();

                println!("  image path: {path}");
                println!("  magic:      {}", ident["magic"]);
                print!("  version:    {}", ident["version"]);

                if ident["version"] != Mu::VERSION {
                    println!(
                        "    ! warning: version mismatch, {} expected {}",
                        ident["version"],
                        Mu::VERSION
                    )
                } else {
                    println!()
                }

                option = Some(Image {
                    magic: ident["magic"].to_string(),
                    version: ident["version"].to_string(),
                });
            }
            None => {
                println!("     ! error: .ident section not found");
                return None;
            }
        }

        match reader.section_by_name(".image") {
            Some(section) => {
                let size = reader.section_data(section).unwrap().len();

                println!("  image size: {size:?}")
            }
            None => {
                println!("     ! error: .image section not found");
                return None;
            }
        }

        option
    }

    pub fn write_image(env: &Env, path: &str) {
        let ident_json = object! {
            magic: Self::IMAGE_MAGIC.to_string(),
            version: Mu::VERSION.to_string(),
        };

        let writer = Writer::with(
            path,
            ident_json.dump().as_bytes().to_vec(),
            Mu::image(env).unwrap(),
        )
        .unwrap();

        writer.write().unwrap()
    }
}

#[cfg(test)]
mod tests {}
