// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ::Ressource;
use ::Status;
use ::serialize::Serialize;

///An async download that represents a `Ressource`
pub trait Download : Sized + Serialize {
    ///The type of the underlying ressource
    type Data : Ressource;

    ///Constructs a new download upon a `Ressource`
    fn new(target : Self::Data) -> Self;

    ///Starts the download if not already running
    fn start(&mut self) -> bool;

    ///Abort the download if running
    fn stop(&mut self) -> bool;

    ///Returns a reference to the underlying `Ressource`
    fn get_ressource(&self) -> &Self::Data;

    ///Returns the current status of the download
    fn get_status(&self) -> Status;
}
