#![allow(unused)] // suppress warnings for unused code
// tutorial video from https://www.youtube.com/watch?v=ygL_xcavzQ4
use std::{io, hash}; // standard library for input/output
use rand::Rng; // use random number range -> needs dependency in Cargo.toml
// import nested things from a crate
use std::io::{Write, BufReader, BufRead, ErrorKind}; // use input/output, buffered reader, buffered read
use std::fs::File;
use std::cmp::Ordering; // use comparison

// we need this do do addition with generics
use std::ops::Add;

// for hashmaps
use std::collections::HashMap;

fn main() {
    // how to assign values out of a multiple return function
    // let (x, y) = returning_multiple_values();
    // println!("x : {}, y : {}\n", x, y);
    
    // summing elements of a list
    // let num_list = [1,2,3,4,5,6,7,8,9,10];
    // println!("sum of list : {}\n", sum_list(&num_list));
    // println!("alternate sum of list : {}\n", alternate_sum_list(&num_list));
    // println!("advanced sum of list : {}\n", advanced_sum_list(&num_list));

    //generic add
    // println!("generic add : {}\n", get_sum_gen(5, 6));
    // println!("generic add : {}\n", get_sum_gen(5.500, 6.534));

    // hashmaps
    // hashy_mappy();

    // using a struct
    // let mut customer1 = Customer {
    //     name: String::from("Johnny Doeboy"),
    //     age: 108,
    //     active: true
    // };

    // customer1.name = String::from("Johnny Doeboy Jr.");
    
    // generic rectangle
    // let rect: Rectangle<i32, f64> = Rectangle {length: 5, width: 6.5};

    
    // using the trait implementation for shape
    let rect: Rectangle = Shape::new(5.0, 6.0);
}

// trait -- sort of like an interface in java
trait Shape {
    fn new(length:f32, width:f32) -> Self; // a contrstuctor for the trait
    fn area(&self) -> f32; // a method for the trait
}

// structs for the shape trait
struct Rectangle {
    length: f32,
    width: f32
}

struct Triangle {
    length: f32,
    width: f32
}

// implementation of the shape trait for the rectangle struct
impl Shape for Rectangle{
    fn new(length: f32, width: f32) -> Rectangle {
        Rectangle {length, width}
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }
}

//generic struct
// struct Rectangle<T,U> {
//     length: T,
//     width: U
// }

// normal struct
struct Customer {
    name: String,
    age: u8,
    active: bool
}

fn hashy_mappy() {
    let mut heroes = HashMap::new();
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Spiderman", "Peter Parker");

    for (k, v) in heroes.iter(){
        println!("{} is {}", k, v);
    }
}

// utilizing the add "TRAIT"
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn advanced_sum_list(list: &[i32]) -> i32 {
    list.iter().fold(0, |sum, x| sum + x)
}

fn alternate_sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }

    sum
}

fn builtin_sum_list(list: &[i32]) -> i32 {
    list.iter().sum()
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum
}

fn returning_multiple_values() -> (i32, i32) {
    (5, 6)
}

fn returning_func() -> i32 {
    5
}

fn second_returning_func() ->  i32 {
    return 5
}

fn simple_func() {
    println!("Hello, world!");
}

fn parameterized_func(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn simple_vector_ops() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5,6,7,8,9,10];

    vec2.push(5);
    println!("vec2 : {:?}", vec2);
    println!("vec2[0] : {}", vec2[0]);

    // check if value exists in a vector
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("second : {}", second),
        None => println!("There is no second element")
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("i : {}", i);
    }

    println!("Vec len: {}", vec2.len());
    // can also pop last value
    println!("Popped val : {}", vec2.pop().unwrap());
}

fn enums_enumerated_types() {
    enum Days {
        MONDAY,
        TUESDAY,
        WEDNESDAY,
        THURSDAY,
        FRIDAY,
        SATURDAY,
        SUNDAY
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::SATURDAY | Days::SUNDAY => true,
                _ => false
            }
        }
    }

    let today: Days = Days::MONDAY;
    match today {
        Days::MONDAY => println!("Monday"),
        Days::TUESDAY => println!("Tuesday"),
        Days::WEDNESDAY => println!("Wednesday"),
        Days::THURSDAY => println!("Thursday"),
        Days::FRIDAY => println!("Friday"),
        Days::SATURDAY => println!("Saturday"),
        Days::SUNDAY => println!("Sunday"),
    }

    println!("Is today a weekend? {}", today.is_weekend());
}

fn casting_example() {
    // casting example
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn more_string_ops() {
    let str1 = String::from("c j d l e i s h a k l l m e");
    // turns a string into a byte array
    let byte_arr = str1.as_bytes();

    // get a str slice from a string
    let str_slice = &str1[0..5];

    // combine strings
    let st2 = String::from("Hello");
    let st3 = String::from("World");
    let st4 = st2 + &st3; // st2 has been moved here and can no longer be used, st3 is borrowed

}

fn make_string_pop_char2_vector_sort() {
    let str1 = String::from("a f j i g t j i j g f j");

    let mut v1: Vec<char> = str1.chars().collect();

    v1.sort();
    v1.dedup(); // remove duplicates

    for char in v1 {
        println!("char : {}", char);
    }

}

fn push_to_mutable_string_and_forloop() {

    // mutable growable string
    let mut str1: String = String::new();
    str1.push_str("Hello");
    str1.push('A');
    str1.push_str(" World");

    for word in str1.split_whitespace() {
        println!("word : {}", word);
    }
}

fn tuples_example() {
    // tuples can contain different types of data
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2: (u32, String, f64) = (500, String::from("Hello"), 6.4);

    println!("tup1 : {:?}", tup1);
    println!("tup2 : {:?}", tup2);
    println!("tup1.0 : {}", tup1.0);
    println!("tup2.1 : {}", tup2.1);
    println!("tup2.1.len() : {}", tup2.1.len());

}

fn for_loop_over_array_1() {
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let mut loopdex = 0;

    for val in arr.iter(){
        println!("val : {}", val);
    }
}

fn while_loop_over_array_1(){
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let mut loopdex = 0;

    while loopdex < arr.len(){
        println!("val : {}", arr[loopdex]);
        loopdex += 1;
    }
}

fn loop_over_array_2(){
    let arr: [u128; 7] = [1324,2345,3456,4567,5678,6789,7890];
    let mut loopdex = 0;
    loop{

        if loopdex >= 6{
            break;
        }

        if arr[loopdex] % 2 == 0 {
            loopdex += 1;
            continue;
        }

        println!("val : {}", arr[loopdex]);
        loopdex += 1;
    }
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
