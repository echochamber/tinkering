#![feature(io)]
extern crate termion;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

extern crate tokio_core;

use std::io::{Write, Read, BufRead, Cursor, self};
use termion::raw::IntoRawMode;
use std::char::from_u32;


fn run() -> io::Result<()> {
    let echo_ed = clap_app!(
        echo_ed =>
        (about: "Echochamber Editor")
        (@arg irrel_arg: -i --irel +takes_value "Irrelevant arg, wanted to include an arg")
    );

    Ok(())

    // let mut core = tokio_core::reactor::Core::new().chain_err(|| "Error creating event loop.");
}

fn read_by_char_working() -> io::Result<()> {
    let mut buffer = String::new();

    let mut continue_reading = true;
    for my_char in io::stdin().chars() {
        let a_char = my_char.unwrap();
        if a_char == 'q' {
            break;
        } else {
            io::stdout().write(a_char.clone().to_string().into_bytes().as_slice());
        }

    }

    return Ok(());
}

fn iotesting() -> io::Result<()> {

    for byte in io::stdin().bytes() {
        let item = byte.unwrap();
        if item == b'q' {
            break;
        } else {
            io::stdout().write(&[item]);
            io::stdout().flush();
        }
    }


    return Ok(());
}

fn main() {
    iotesting();
    // io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
}
