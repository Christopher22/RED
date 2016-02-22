// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;

pub use self::serde::ser::{Serialize, Serializer};
pub use self::serde::de::{Deserialize, Deserializer, Visitor};
pub use self::serde::de::Error as DeserializeError;
pub use self::serde::de::MapVisitor as DeserializeMapVisitor;
pub use self::serde::ser::MapVisitor as SerializeMapVisitor;

pub struct StructSerializer<'a, T : 'a> {
    state: usize,
    pub value: &'a T
}

impl<'a, T> StructSerializer<'a, T> {
    pub fn new(value : &'a T) -> StructSerializer<'a, T> {
        StructSerializer {
            state : 0,
            value : value
        }
    }

    #[inline]
    pub fn next(&mut self) -> usize {
        self.state += 1;
        self.state - 1
    }
}
