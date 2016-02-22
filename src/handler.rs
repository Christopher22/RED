// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate download;
extern crate hyper;

extern crate serde_json;

use std;
use std::ops::Deref;
use std::sync::Mutex;
use std::fs::File;
use std::path::{PathBuf, Component};

use hyper::server::{Handler, Request, Response};
use hyper::method::Method;
use hyper::header::{CacheDirective, CacheControl, ContentType, ContentLength, Location};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::mime::{Mime, TopLevel, SubLevel};

use self::download::Manager;
use self::download::http::{HttpDownload, HttpRessource};
use self::download::Ressource;

/// Represents a remote download manager
pub struct DownloadServer {
    downloads: Mutex<Manager<HttpDownload>>,
    root_dir: Mutex<std::path::PathBuf>,
    storage_dir: Mutex<std::path::PathBuf>
}

impl DownloadServer {

    // Creates a new server
    pub fn new(root_dir : std::path::PathBuf, storage : std::path::PathBuf) -> DownloadServer {
        DownloadServer {
            root_dir : Mutex::new(root_dir),
            storage_dir : Mutex::new(storage),
            downloads : Mutex::new(Manager::new())
        }
    }

    /// Handles the HTTP-requests for current downloads and add new ones
    fn handle_download(&self, mut url : String, req : Request, mut res : Response) {

        //Disable caching and set type of response
        {
            let mut headers = res.headers_mut();
            headers.set(ContentType::json());
            headers.set(CacheControl(vec![CacheDirective::NoCache, CacheDirective::NoStore, CacheDirective::MustRevalidate]));
        }

        //If all downloads are requested or a new one is to be added ...
        if url.len() == 11 {

            match req.method {

                //If all downloads are requested ...
                Method::Get => {
                    serde_json::to_writer(&mut res.start().unwrap(), self.downloads.lock().unwrap().deref()).expect("Generation of JSON failed!");
                },

                //... or a new one is to be added ...
                Method::Post => {
                    if let Ok(mut value) = serde_json::de::from_reader::<Request, HttpRessource>(req) {

                        //If the path is save ...
                        if value.path().is_relative() && value.path().components().position(|x| if let Component::ParentDir = x { true } else { false } ).is_none() {
                            let new_path = self.storage_dir.lock().unwrap().join(value.path());

                            value.set_path(new_path);
                            url.push_str(&self.downloads.lock().unwrap().add(value).to_string());

                            (*res.headers_mut()).set(Location(url));
                            *res.status_mut() = StatusCode::Created;
                        }
                        else {
                            *res.status_mut() = StatusCode::Forbidden;
                        }
                    }
                    else {
                        *res.status_mut() = StatusCode::BadRequest;
                    }
                    res.send("{}".as_bytes()).unwrap();
                },

                //... or an unknown method is used.
                _ => {
                    *res.status_mut() = StatusCode::MethodNotAllowed;
                    res.send("{}".as_bytes()).unwrap();
                }
            }
        }

        //... or a specific one is requested ...
        else if let Ok(id) = url.chars().skip(11).collect::<String>().parse::<usize>() {
            let mut manager = self.downloads.lock().unwrap();

            if manager.exists(id) {
                match req.method {
                    Method::Get => {
                        serde_json::to_writer(&mut res.start().unwrap(), &manager[id]).expect("Generation of JSON failed!");
                    },
                    Method::Delete => {
                        manager.remove(id);
                    },
                    _ => {
                        *res.status_mut() = StatusCode::MethodNotAllowed;
                        res.send("{}".as_bytes()).unwrap();
                    }
                }
            }
            else {
                *res.status_mut() = StatusCode::NotFound;
                res.send("{}".as_bytes()).unwrap();
            }
        }

        //... or someone type gabage.
        else {
            *res.status_mut() = StatusCode::NotFound;
            res.send("{}".as_bytes()).unwrap();
        }
    }

    /// Serve a file from local filesystem
    fn serve_file(&self, url : String, mut res : Response) {

        //Extract the file name
        let mut target_file = String::from(url.split_at(url.rfind('/').unwrap_or(0) + 1).1);
        if target_file.len() == 0 {
            target_file.push_str("index.html");
        }
        let path = self.root_dir.lock().unwrap().join(target_file);

        //Try to open file
        if let Ok(mut file) = File::open(path.clone()) {
            {
                let mut headers = res.headers_mut();

                //Set the file length, if possible
                if let Ok(meta) = file.metadata() {
                    headers.set(ContentLength(meta.len()));
                };

                //Set the type of file
                if let Some(ext) = path.extension() {
                    headers.set(match ext.to_str().unwrap() {
                        "html" => ContentType::html(),
                        "css" => ContentType(Mime(TopLevel::Text, SubLevel::Css, vec![])),
                        "js" => ContentType(Mime(TopLevel::Text, SubLevel::Javascript, vec![])),
                        _ => ContentType::plaintext()
                    });
                };

                //Enable caching
                headers.set(CacheControl(vec![CacheDirective::MaxAge(86400u32)]));
            }

            //Transload file
            if let Ok(mut stream) = res.start() {
                std::io::copy(&mut file, &mut stream).expect("File transfer failed!");
            }
        }
        else {
            *res.status_mut() = StatusCode::NotFound;
        }
    }
}

impl Handler for DownloadServer {
    fn handle(&self, req: Request, res: Response) {
        if let RequestUri::AbsolutePath(url) = req.uri.clone() {
            if url.starts_with("/downloads/") {
                self.handle_download(url, req, res);
            }
            else {
                self.serve_file(url, res)
            }
        }
    }
}
