// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate vec_map;
use self::vec_map::VecMap;

use std::ops::{Index, IndexMut};

use ::Download;
use ::serialize::*;

///A manager for simultaneous downloads
pub struct Manager<T : Download> {
    counter : usize,
    holes : Vec<usize>,
    downloads : VecMap<T>
}

impl<T : Download> Manager<T> {
    ///Constructs a new `Manager`
    pub fn new() -> Manager<T> {
        Manager {
            counter : 0,
            holes : vec![],
            downloads : VecMap::new()
        }
    }

    ///Adds a new `Download` and returns its ID
    pub fn add(&mut self, ressource : T::Data) -> usize {
        let id = if let Option::Some(old_id) = self.holes.pop() {
            old_id
        }
        else {
            self.counter += 1;
            self.counter - 1
        };

        self.downloads.insert(id, T::new(ressource));
        self.downloads[id].start();
        id
    }

    #[inline]
    ///Returns `true` if the given ID is valid, else `false`
    pub fn exists(&self, id : usize) -> bool {
        self.downloads.contains_key(id)
    }

    ///Removes the `Download` and returns `true`if the opeation was successfull
    pub fn remove(&mut self, id : usize) -> bool {
        if let Option::None = self.downloads.remove(id) {
            false
        }
        else {
            self.holes.push(id);
            true
        }
    }
}

impl<T : Download> Index<usize> for Manager<T> {
    type Output = T;

    #[inline]
    fn index<'a>(&'a self, index: usize) -> &'a T {
        self.downloads.index(index)
    }
}

impl<T : Download> IndexMut<usize> for Manager<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        self.downloads.index_mut(index)
    }
}

impl<T : Download> Serialize for Manager<T> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.visit_map(StructSerializer::new(self))
    }
}

impl<'a, T : Download> SerializeMapVisitor for StructSerializer<'a, Manager<T>> {
    fn visit<S: Serializer>(&mut self, visitor: &mut S) -> Result<Option<()>, S::Error> {
        let mut current = self.next();
        while !self.value.exists(current) && current < self.value.counter {
            current = self.next();
        }

        if current == self.value.counter {
            Ok(None)
        }
        else {
            Ok(Some(try!(visitor.visit_map_elt(current.to_string(), &self.value.index(current)))))
        }

    }
}
