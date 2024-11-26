//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! env nursery interface
#[allow(unused_imports)]
use crate::{
    core::{
        direct::DirectTag,
        env::Env,
        exception,
        frame::Frame,
        indirect::{self, IndirectTag},
        types::{ImageId, Tag, Type, TypeImage},
    },
    types::{
        cons::{Cons, Core as _},
        fixnum::{Core as _, Fixnum},
        function::{Core as _, Function},
        struct_::{Core as _, Struct},
        symbol::{Core as _, Symbol},
        vector::Vector,
    },
    vectors::core::Core as _,
};

use std::collections::HashMap;

// locking protocols
use futures::executor::block_on;
use futures_locks::RwLock;

impl Default for Nursery {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Nursery {
    id: RwLock<ImageId>,
    map: RwLock<HashMap<ImageId, TypeImage>>,
}

impl Nursery {
    pub fn new() -> Self {
        Nursery {
            id: RwLock::new(0),
            map: RwLock::new(HashMap::new()),
        }
    }
}

pub trait Core {
    fn nursery_info(&self) -> usize;
    fn nursery_ref(&self, _: &ImageId) -> TypeImage;
    fn nursery_cache_image(&self, _: TypeImage);
    fn nursery_uncache_image(&self, _: &ImageId);
}

impl Core for Nursery {
    fn nursery_info(&self) -> usize {
        let map_ref = block_on(self.map.read());

        (*map_ref).len()
    }

    fn nursery_ref(&self, id: &ImageId) -> TypeImage {
        let map_ref = block_on(self.map.read());

        map_ref[id].clone()
    }

    fn nursery_cache_image(&self, image: TypeImage) {
        let mut map_ref = block_on(self.map.write());
        let mut nursery_id = block_on(self.id.write());

        let id = *nursery_id;

        *nursery_id = id + 1;
        map_ref.insert(id, image).unwrap();
    }

    fn nursery_uncache_image(&self, id: &ImageId) {
        let mut map_ref = block_on(self.map.write());

        map_ref.remove(id).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn env() {
        assert_eq!(2 + 2, 4);
    }
}
