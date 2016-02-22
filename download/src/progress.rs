// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate time;

#[derive(Debug, Clone)]
///The current progress of a running download
pub struct Progress {
    current : u64,
    max : u64
}

impl Progress {
    ///Creates a non-completed `Progress`
    pub fn new(current : u64, max : u64) -> Option<Progress> {
        if max >= current {
            Some(Progress {
                current : current,
                max : max
            })
        }
        else {
            None
        }
    }

    #[inline]
    ///Adds an amount to the progress
    pub fn add(&mut self, value : u64) {
        self.current += value;
    }

    #[inline]
    ///Gets the current progress
    pub fn current(&self) -> u64 {
        self.current
    }

    #[inline]
    ///Sets the maximum of the progress
    pub fn set_max(&mut self, value : u64) {
        self.max = value;
    }

    #[inline]
    ///Gets the maximum of the progress
    pub fn max(&self) -> u64 {
        self.max
    }

    #[inline]
    ///Returns `true` if the progress reached its maximum
    pub fn is_done(&self) -> bool {
        self.current >= self.max
    }

    #[inline]
    ///Returns the current progress in percent
    // pub fn get_percent(&self) -> u8  {
    //     ((self.current / self.max) * 100) as u8
    // }
    pub fn get_percent(&self) -> f64  {
        (self.current as f64 / self.max as f64) * 100.0
    }
}
