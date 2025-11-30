//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// vector cache
use {
    crate::{
        core::{env::Env, tag::Tag, type_::Type},
        types::{fixnum::Fixnum, float::Float, vector::Vector},
        vectors::image::{VecImageType, VectorImageType},
    },
    futures_lite::future::block_on,
    futures_locks::RwLock,
    std::collections::HashMap,
};

pub type VecCacheMap = HashMap<(Type, i32), RwLock<Vec<Tag>>>;

impl Vector {
    pub fn cache(env: &Env, vector: Tag) {
        let vtype = Self::type_of(env, vector).map_type();
        let length = i32::try_from(Self::length(env, vector)).unwrap();
        let mut cache = block_on(env.vector_cache.write());

        match (*cache).get(&(vtype, length)) {
            Some(vec_map) => {
                let mut vec = block_on(vec_map.write());

                vec.push(vector);
            }
            None => {
                if (*cache)
                    .insert((vtype, length), RwLock::new(vec![vector]))
                    .is_some()
                {
                    panic!()
                }
            }
        }
    }

    pub fn cached(env: &Env, indirect: &VecImageType) -> Option<Tag> {
        let cache = block_on(env.vector_cache.read());

        let (vtype, length, ivec) = match indirect {
            VecImageType::Bit(image, ivec)
            | VecImageType::Byte(image, ivec)
            | VecImageType::Char(image, ivec)
            | VecImageType::Fixnum(image, ivec)
            | VecImageType::Float(image, ivec) => (
                image.type_,
                i32::try_from(Fixnum::as_i64(image.length)).unwrap(),
                ivec,
            ),
            VecImageType::T(_, _) => panic!(),
        };

        match (*cache).get(&(vtype.key_to_type().unwrap(), length)) {
            Some(vec_map) => {
                let tag_vec = block_on(vec_map.read());

                let tag = match ivec {
                    VectorImageType::Bit(u8_vec) => tag_vec.iter().find(|src| {
                        u8_vec.iter().enumerate().all(|(index, byte)| {
                            i64::from(*byte)
                                == Fixnum::as_i64(Vector::ref_(env, **src, index).unwrap())
                        })
                    }),
                    VectorImageType::Byte(u8_vec) => tag_vec.iter().find(|src| {
                        u8_vec.iter().enumerate().all(|(index, byte)| {
                            i64::from(*byte)
                                == Fixnum::as_i64(Vector::ref_(env, **src, index).unwrap())
                        })
                    }),
                    VectorImageType::Char(string) => tag_vec
                        .iter()
                        .find(|src| *string == Vector::as_string(env, **src)),
                    VectorImageType::Fixnum(i64_vec) => tag_vec.iter().find(|src| {
                        i64_vec.iter().enumerate().all(|(index, fixnum)| {
                            *fixnum == Fixnum::as_i64(Vector::ref_(env, **src, index).unwrap())
                        })
                    }),
                    VectorImageType::Float(float_vec) => tag_vec.iter().find(|src| {
                        #[allow(clippy::float_cmp)]
                        float_vec.iter().enumerate().all(|(index, float)| {
                            *float == Float::as_f32(env, Vector::ref_(env, **src, index).unwrap())
                        })
                    }),
                    VectorImageType::T(_) => panic!(),
                };

                tag.copied()
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
