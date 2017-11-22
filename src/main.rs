#![feature(io)]
extern crate termion;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate error_chain;

extern crate tokio_core;

use std::io::{Write, Read, BufRead, Cursor, self};
use termion::raw::{RawTerminal, IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;
use std::char::from_u32;

struct Backend {
    buffer: String
}

struct Frontend {
    backend: Backend,
    stdout: RawTerminal<io::Stdout>
}

impl Frontend {
    fn write_frame(&mut self) {
        write!(self.stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), &self.backend.buffer).unwrap();
        self.stdout.flush().unwrap();
    }

    fn run(&mut self) {
        let mut stdin = io::stdin();
        for maybe_key in stdin.keys() {
            match maybe_key {
                Ok(Key::Char(c)) => {
                    self.backend.buffer.push(c);
                    self.write_frame();
                },
                Ok(Key::Backspace) => {
                    self.backend.buffer.pop();
                    self.write_frame();
                },
                Ok(Key::Esc) => {
                    break;
                },
                Ok(_) => {},
                _ => {},
            }
        }
    }
}



fn read_stdin_raw() {
    let mut frontend = Frontend {
        backend: Backend { buffer: String::new() },
        stdout: io::stdout().into_raw_mode().unwrap()
    };
    frontend.run();
}

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
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut continue_reading = true;
    for my_char in io::stdin().chars() {
        let a_char = my_char.unwrap();
        if a_char == 'q' {
            break;
        } else {
            stdout.write(a_char.clone().to_string().into_bytes().as_slice());
        }

    }

    return Ok(());
}

fn iotesting() -> io::Result<()> {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let item = byte.unwrap();
        if item == b'q' {
            break;
        } else {
            stdout.write(&[item]);
            stdout.flush();
        }
    }


    return Ok(());
}

fn main() {
    //iotesting();
    // io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();

    read_stdin_raw();
}
