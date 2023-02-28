#![allow(unused)] // suppress warnings for unused code

use std::io; // standard library for input/output
use rand::Rng; // use random number range -> needs dependency in Cargo.toml
// import nested things from a crate
use std::io::{Write, BufReader, BufRead, ErrorKind}; // use input/output, buffered reader, buffered read
use std::fs::File;
use std::cmp::Ordering; // use comparison

fn main() {
    println!("Hello, world!");
}
