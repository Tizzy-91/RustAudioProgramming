#![allow(unused)] // suppress warnings for unused code
// tutorial video from https://www.youtube.com/watch?v=ygL_xcavzQ4
use std::{io, hash}; // standard library for input/output
// use rand::Rng; // use random number range -> needs dependency in Cargo.toml
// import nested things from a crate
use std::io::{Write, BufReader, BufRead, ErrorKind}; // use input/output, buffered reader, buffered read
use std::fs::File;
use std::cmp::Ordering; // use comparison

fn main() {
    let mut arr_it = [1,2,3,4];

    for val in arr_it.iter(){
        println!("{}", val);
    }

    // consumes the collection, but we can no longer use it.
    arr_it.into_iter();

    let mut it = arr_it.iter(); // iterator for the array, we can cycle through em now
    println!("1st element : {:?}", it.next());
}
