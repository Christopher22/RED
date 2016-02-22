// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use ::Progress;
use ::Error;

#[derive(Debug, Clone)]
///Represents the current status of a `Download`
pub enum Status {
    ///The `Download` waits to get started
    Waiting,
    ///The `Download` prepares itself
    Loading,
    ///The `Download` is running
    Running(Option<Progress>),
    ///The `Download` finished successfully
    Finished,
    ///The `Download` was aborted
    Aborted(Error)
}

impl fmt::Display for Status {
    fn fmt(&self, fm : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (match *self {
            Status::Waiting => fm.write_str("Ready"),
            Status::Loading => fm.write_str("Loading"),
            Status::Running(ref progress) => {
                if let &Some(ref data) = progress {
                    fm.write_str(&format!("Running: {:03.2}%", data.get_percent()))
                }
                else {
                    fm.write_str("Running")
                }
            }
            Status::Finished => fm.write_str("Finished"),
            Status::Aborted(ref reason) => reason.fmt(fm)
        }).expect("Generating a status failed!");
        Ok(())
    }
}

impl ::serialize::Serialize for Status {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ::serialize::Serializer,
    {
        serializer.visit_str(&format!("{}", *self))
    }
}
