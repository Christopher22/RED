// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate url;
pub use self::url::Url as Url;

use std::path::{Path, PathBuf};

use ::serialize::{Serialize, Deserialize};

///Represents the used protocol, target, destination and probably other options of an `Download`
pub trait Ressource : Sized + Serialize + Deserialize {
    ///Creates a new `Ressource`
    fn new(url : Url, path : PathBuf) -> Option<Self>;

    ///Returns the source
    fn url(&self) -> &Url;

    //Returns the (local) target
    fn path(&self) -> &Path;
}
