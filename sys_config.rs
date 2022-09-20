use std::fs::File;
use std::io::prelude::*;

#[allow(missing_docs)]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // update system to unlimited for crawler
    let mut file = File::create("unlimit")?;
    file.write(b"#!/bin/sh\n")?;
    file.write(b"\n")?;
    // max open files
    file.write(b"ulimit -n 999999\n")?;
    // cpu time unlimited
    file.write(b"ulimit -t unlimited\n")?;
    // mem unlimited
    file.write(b"ulimit -m unlimited\n")?;
    // blocks unlimited
    file.write(b"ulimit -f unlimited\n")?;
    // stack process
    file.write(b"ulimit -s unlimited\n")?;
    // virtual mem
    file.write(b"ulimit -v unlimited\n")?;
    // // user process
    // file.write(b"ulimit -u unlimited\n")?;
    // // open files
    // file.write(b"ulimit -n unlimited\n")?;

    Ok(())
}
