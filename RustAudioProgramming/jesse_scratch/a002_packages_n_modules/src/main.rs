// more tutorial from same video: https://www.youtube.com/watch?v=ygL_xcavzQ4
// Crates : modules that produce library or executable
// Modules : organize and handle privacy
// packages : build, test, and share crates
// paths: a way of naming an item, such as a struct, function, or module

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // using the restaurant crate
    // println!("Hello, world!");
    // order_food();

    // going through error stuff
    panic!("crash and burn");
}
