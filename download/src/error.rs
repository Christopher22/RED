// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{error, fmt};

#[derive(Debug, Clone)]
///Represents the reason for aborting a `Download`
pub enum Error {
    ///The user aborted the `Download`
    User,
    ///An error occurs on requesting the source
    Request,
    ///An error occurs on recieving from the source
    Response,
    ///An error occurs on creating the (local) target
    CreateFile,
    ///An error occurs on writing to the (local) target
    WriteFile
}

impl fmt::Display for Error {
    fn fmt(&self, fm : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        (match *self {
            Error::User => fm.write_str("Aborted"),
            Error::CreateFile => fm.write_str("Error: Saving failed"),
            Error::WriteFile => fm.write_str("Error: Writing failed"),
            Error::Request => fm.write_str("Error: Requesting ressource"),
            Error::Response => fm.write_str("Error: Recieving ressource")
        }).expect("Generating an error failed!");
        Ok(())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "The download was aborted"
    }
}

impl ::serialize::Serialize for Error {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ::serialize::Serializer,
    {
        serializer.visit_str(&format!("{}", *self))
    }
}
