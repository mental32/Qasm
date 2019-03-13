extern crate clap;
extern crate pest;

use clap::{Arg, App};

use std::{
  io::Read,
  fs::File
};

mod parser;

fn main() {
    let matches = App::new("Qasm")
      .version("0.1.0")
      .author("mental <m3nta1@yahoo.com>")
      .about("The polite assembler")
      .arg(Arg::with_name("source")
           .help("The source file to assemble")
           .required(true)
           .index(1))
      .get_matches();

    let outfn = matches.value_of("output").unwrap_or("a.out");
    let mut source = String::new();

    File::open(matches.value_of("source").unwrap())
        .expect("Could not open source file!")
        .read_to_string(&mut source)
        .expect("Could not read from file!");

    parser::parse(&source);

    let outf = match File::create(outfn) {
        Ok(file) => file,
        Err(err) => panic!(err)
    };
}
