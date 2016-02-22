// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
use self::hyper::client::Client;
use self::hyper::header::ContentLength;

use std::fs::File;
use std::thread;
use std::ops::Deref;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use ::Ressource;
use ::Download;
use ::Status;
use ::Error;
use ::Progress;
use ::serialize::*;

use super::HttpRessource;

pub const BUFFER_SIZE : usize = 1024;

pub struct HttpDownload {
    data : Arc<HttpRessource>,
    status : Arc<Mutex<Status>>,
    running : Arc<AtomicBool>
}

impl HttpDownload {
    fn spawn_thread(data : Arc<HttpRessource>, status : Arc<Mutex<Status>>, running : Arc<AtomicBool>) {
        thread::spawn(move|| {

            let client = Client::new();
            let request = client.get(data.url().clone());

            //Send request
            if let Ok(ref mut response) = request.send() {
                let mut buffer = [0; BUFFER_SIZE];

                //Create local file
                if let Ok(ref mut file_handle) = File::create(data.path()) {

                    //Update the status and calculate progress, if possible
                    *status.lock().unwrap() = Status::Running(Progress::new(0, if let Some(l) = response.headers.get::<ContentLength>() { l.deref().clone() } else { 0 }));

                    //Read response chunk by chunk
                    while let Ok(d) = response.read(&mut buffer) {

                        //If the download is done, ...
                        if d == 0 {
                            break;
                        }

                        //... an error occurres, ...
                        else if let Err(_) = file_handle.write_all(if d == BUFFER_SIZE { &buffer } else { &buffer[..d]}) {
                            *status.lock().unwrap() = Status::Aborted(Error::WriteFile);
                            return;
                        }

                        //... or the download is running successfull.
                        else if let Status::Running(ref mut progress_option) = *status.lock().unwrap() { //Update progress
                            if let Some(ref mut progress) = *progress_option {
                                progress.add(d as u64);
                            }
                        }

                        //If the download is going to be aborted
                        if running.load(Ordering::Relaxed) == false { //Should abort?!
                            *status.lock().unwrap() = Status::Aborted(Error::User);
                            return;
                        }
                    }

                    *status.lock().unwrap() = Status::Finished;
                }
                else {
                    *status.lock().unwrap() = Status::Aborted(Error::CreateFile);
                }
            }
            else {
                *status.lock().unwrap() = Status::Aborted(Error::Request);
            }
        });
    }
}

impl super::super::Download for HttpDownload {
    type Data = HttpRessource;

    fn new(target : HttpRessource) -> HttpDownload {
        HttpDownload {
            data : Arc::new(target),
            running : Arc::new(AtomicBool::new(false)),
            status : Arc::new(Mutex::new(Status::Waiting))
        }
    }

    fn start(&mut self) -> bool {
        //If the download is not already running...
        if !self.running.load(Ordering::Relaxed) {
            self.running.store(true, Ordering::Relaxed);
            *self.status.lock().unwrap() = Status::Loading;

            //Start the download
            HttpDownload::spawn_thread(self.data.clone(), self.status.clone(), self.running.clone());
            true
        }
        else {
            false
        }
    }

    fn stop(&mut self) -> bool {
        if self.running.load(Ordering::Relaxed) {
            self.running.store(false, Ordering::Relaxed);
            true
        }
        else {
            false
        }
    }

    fn get_ressource(&self) -> &HttpRessource {
        self.data.deref()
    }

    fn get_status(&self) -> Status {
        (*self.status.lock().unwrap()).clone()
    }
}

impl Drop for HttpDownload {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Serialize for HttpDownload {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.visit_struct("Download", StructSerializer::new(self))
    }
}

impl<'a> SerializeMapVisitor for StructSerializer<'a, HttpDownload> {
    fn visit<S: Serializer>(&mut self, visitor: &mut S) -> Result<Option<()>, S::Error> {
        match self.next() {
            0 => Ok(Some(try!(visitor.visit_map_elt("Ressource", &self.value.get_ressource())))),
            1 => Ok(Some(try!(visitor.visit_map_elt("Status", &self.value.status.lock().unwrap().deref())))),
            _ => Ok(None)
        }
    }
}
