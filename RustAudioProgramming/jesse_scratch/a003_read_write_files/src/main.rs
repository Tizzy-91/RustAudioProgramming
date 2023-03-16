#![allow(unused)] // suppress warnings for unused code
// tutorial video from https://www.youtube.com/watch?v=ygL_xcavzQ4
use std::{io, hash}; // standard library for input/output
use rand::Rng; // use random number range -> needs dependency in Cargo.toml
// import nested things from a crate
use std::io::{Write, BufReader, BufRead, ErrorKind}; // use input/output, buffered reader, buffered read
use std::fs::File;
use std::cmp::Ordering; // use comparison

fn file_example() {
    let path: &str = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem with file creation: {:?}", error)
    };

    write!(output, "this is a line\nthis is line 2").expect("failed to write to file");

    let input = File::open(path).unwrap(); // unwrap ignores result and just gives the output
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }

    // handling multiple specific errors
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", _other_error),
        },
    };

}

fn main() {
    file_example();
}
