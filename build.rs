// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs::*;
use std::path::PathBuf;

fn main() {
    let mut source = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    source.push("web");

    //Get root of project
    let mut target = PathBuf::from(env::var("OUT_DIR").unwrap());
    target.pop();
    target.pop();
    target.pop();
    target.push("web");

    //Delete directory, if already existing
    if std::fs::metadata(&target).ok().map(|t| t.is_dir()).unwrap_or(false)  {
        std::fs::remove_dir_all(&target).expect("Removal of directory failed!");
    }

    copy_dir(&source, &target);
}

///Copy a directory inclusive subdirectories
fn copy_dir(source : &PathBuf, target : &PathBuf) {
    std::fs::create_dir(target).expect("Creation of directory failed!");

    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let entry_type = std::fs::metadata(entry.path()).unwrap().file_type();

        if entry_type.is_dir() {
            copy_dir(&entry.path(), &target.join(entry.file_name()));
        }
        else if entry_type.is_file() {
            std::fs::copy(&entry.path(), &target.join(entry.file_name())).expect("Copying a file failed!");
        }
    }
}
