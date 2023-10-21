use std::process;
use std::io::{self, Write};


use crate::compiler::read_compile_from_file;

pub mod compiler;

fn main() -> io::Result<()>  {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        writeln!(io::stderr(), "Usage: {} filename", args[0])?;
        process::exit(1);
    }
    let filename = &args[1];

    return read_compile_from_file(filename);
}
