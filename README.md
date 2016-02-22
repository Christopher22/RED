# R.E.D. 
R.E.D. is a small and lightweight downloader for files using HTTP. Originally written for an Raspberry Pi it's controlled by a remote control in the local network.

[![Build Status](https://travis-ci.org/Christopher22/RED.svg?branch=master)](https://travis-ci.org/Christopher22/RED)

##Features:
* Written in pure Rust - no additional library (i.e. curl) needed
* Fast & lightweight
* Build for huge downloads in the background

##Installation:
1. Install Rust and Cargo
2. Download the repository
3. Navigate into the repository and run "cargo build --release"

##Usage:
Choose a folder where to store all the files and start R.E.D. with it as an command line argument. For an overview over its options start RED with the argument "--help". Open your browser and navigate to "http://[IP of device]:8080" (you can specify the port using "--port <port number>"). Add downloads by clicking "Add" and remove/abort them by clicking on there name.

##Author
Christopher Gundler (<c.gundler@mail.de>)

##License
Licensed under either of
 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

##Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
