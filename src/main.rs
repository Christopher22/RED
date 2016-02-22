// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate hyper;
extern crate regex;

use hyper::server::Server;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::str::FromStr;
use std::path::PathBuf;
use std::io::Read;

mod handler;

fn main() {
    let mut args = std::env::args();

    let name = args.next().unwrap_or(String::from("RED"));
    let mut destination = PathBuf::new();

    //Process help or path
    match args.next() {
        Some(ref help) if regex::is_match("^-{0,2}h(elp)?$", &help).unwrap_or(false) => {
            print_help(&name);
        }
        Some(ref path) => {
            destination.push(path);
            if !destination.is_dir() {
                println!("ERROR: Path does not exist!");
                std::process::exit(0);
            }
        },
        None => {
            print_help(&name);
        }
    }

    let url = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), parse_port(&mut args));
    if let Ok(server) = Server::http(url) {
        let handler = handler::DownloadServer::new(std::env::current_dir().unwrap().join("web"), destination);

        if let Ok(mut socket) = server.handle(handler) {
            println!("Press return to exit... ");
            std::io::stdin().read(&mut [0u8]).unwrap();

            socket.close().unwrap();
        }
        else {
            println!("ERROR: Starting server failed!");
        }
    }
    else {
        println!("ERROR: Binding to port {} failed!", url.port());
    }
}

/// Prints important information for the user
fn print_help(name : &str) {
    println!("USAGE:\t {} <path to store files> [(--port | -p) <port number>]", name);
    println!("\t {} -h | --help", name);
    std::process::exit(0);
}

/// Parses the port, if specified
fn parse_port(args : &mut std::env::Args) -> u16 {
    if let Some(option) = args.next() {
        if regex::is_match("^-{0,2}p(ort)?$", &option).unwrap_or(false) && args.size_hint().0 > 0 {
            if let Ok(port) = u16::from_str(&args.next().unwrap_or(String::from("error"))) {
                return port;
            }
        }

        println!("WARNING: Port invalid. Using 8080 instead.");
    }
    8080
}
