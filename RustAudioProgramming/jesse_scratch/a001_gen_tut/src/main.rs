#![allow(unused)] // suppress warnings for unused code
// tutorial video from https://www.youtube.com/watch?v=ygL_xcavzQ4
use std::io; // standard library for input/output
use rand::Rng; // use random number range -> needs dependency in Cargo.toml
// import nested things from a crate
use std::io::{Write, BufReader, BufRead, ErrorKind}; // use input/output, buffered reader, buffered read
use std::fs::File;
use std::cmp::Ordering; // use comparison

fn main(){
    
}

fn loop_over_array_1() {
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("length : {}", arr_1.len());

    let mut loop_index = 0;
    loop {
        if arr_1[loop_index] % 2 == 0{
            println!("{} is even", arr_1[loop_index]);

        } else {
            println!("{} is odd", arr_1[loop_index]);
            
        }
        loop_index += 1;
        if (loop_index == arr_1.len()){
            break;
        }
        continue;
    }

}

fn match_flow_control_comparison() {
    let age = 18;
    let voting_age = 18;
    let drinking_age = 19;
    // like in C, the ampersand is a reference to the variable
    match age.cmp(&voting_age) {
        Ordering::Less => println!("You can't vote"),
        Ordering::Greater => println!("You can vote"),
        Ordering::Equal => println!("You can vote"),
    };
}

fn match_flow_control_ranges() {
    let age: i32 = 99;
    match age{
        1..=18 => println!("You are a child"),
        19..=64 => println!("You are an adult"),
        65..=i32::MAX => println!("You are a senior citizen"),
        _ => println!("You are not born yet")
    };
}

fn conditional_assignment() {
    let age: i32 = 99;
    let is_old: bool = if age >=65 {
        true
    } else {
        false
    };
}

fn basic_if_else() {
    // flow control example
    let age: i32 = 18;
    if age >= 19 {
        println!("You can drink alcohol");
    } else if age >= 18 {
        println!("You can vote");
    } else {
        println!("You can't do anything");
    }
}

fn random_number() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number : {}\n", random_num);
}

fn data_type_exploration() {
    // the types of variables and datatypes in rust
    // unsigned integer : u8, u16, u32, u64, u128, usize
    // signed integer : i8, i16, i32, i64, i128, isize
    // floating point : f32, f64
    // boolean : bool
    // character : char
    // string : &str, String
    // tuple : (i32, f64, u8)
    // array : [i32; 5]
    // slice : &[i32]
    // vector : Vec<i32>
    // hash map : HashMap<K, V>
    // hash set : HashSet<T>

    // maximum values the datatypes can hold
    println!("MAX u32 : {}", u32::MAX);
    println!("MAX i32 : {}", i32::MAX);
    println!("MAX f32 : {}", f32::MAX);
    println!("MAX f64 : {}", f64::MAX);
    println!("MAX usize : {}", usize::MAX);
    println!("MAX isize : {}\n", isize::MAX);

    // booleans
    let is_rust_cool: bool = true;
    println!("Is rust cool? {}\n", is_rust_cool);

    // characters
    let a: char = 'a'; // single quotes for char, just like other langs
    println!("a is {}\n", a);

    // floats
    let x: f32 = 1.111111111111111; // 15 1's after decimal point
    let y: f64 = 1.111111111111111;
    println!("x is {}, y is {}", x, y);
    println!("Truncation of f32 variable results in a rounding up, when it shouldnt have.\n");

    // print the math operators
    let a: i32 = 5;
    let b: i32 = 6;
    println!("Addition : 5 + 6 = {}", a + b);
    println!("Subtraction : 5 - 6 = {}", a - b);
    println!("Multiplication : 5 * 6 = {}", a * b);
    println!("Division : 5 / 6 = {}", a / b);
    println!("Remainder : 5 % 6 = {}\n", a % b);

}

fn variable_name_shadowing() {
    const ONE_MIL: u32 = 1_000_000; // constant
    const PI: f32 = 3.141592; // constant

    // variable with same name but different data types is allowed in rust
    let age: &str = "30";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number"); // mutable variable

    age = age + 1;
    println!("my age is {}, and I want {} dollars", age, ONE_MIL);
}

fn greetings_name_request() {
    println!("What is your name?");
    // mutable string, because by default all vars are immutable
    let mut name: String = String::new(); // String::new() returns an empty string
    let greeting = "Nice to meet you, "; // &str is a string slice
    // recieve input using stdin() and read_line()
    io::stdin().read_line(&mut name).expect("Did not recieve input");
    println!("{}{}", greeting, name);
    
}
