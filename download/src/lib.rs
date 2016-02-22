// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::download::Download;
pub use self::status::Status;
pub use self::error::Error;
pub use self::ressource::{Url, Ressource};
pub use self::progress::Progress;
pub use self::manager::Manager;

mod download;
mod ressource;
mod status;
mod progress;
mod serialize;
mod manager;
mod error;

#[cfg(feature="http")]
pub mod http;
