
// a closure is a function without a name
// way to pass functions into other functions
// reminds me of lambdas in python

// format is 
// let var_name = |param1, param2, ...| -> return_type { code }
fn main() {
    let can_vote = |age: i32| -> bool {
        if age >= 18 {
            true
        } else {
            false
        }
    };

    println!("Can vote: {}", can_vote(19));
    println!("Can vote: {}", can_vote(17));

    // closures can access variables outside of its body unlike a function
    let mut samp1 = 5;
    let print_var = || println!("samp1: {}", samp1);
    print_var();

    samp1 = 10;
    // you can change values if you mark a closure as mutable
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1: {}", samp1);

    fn use_func(f: fn(i32) -> bool, age: i32) {
        println!("Can vote: {}", f(age));
    }

    use_func(can_vote, 19);

    fn use_generic_func <T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a,b| a+b;
    let product = |a,b| a*b;
    println!("Sum: {}", use_generic_func(5, 6, sum));
    println!("Product: {}", use_generic_func(5, 6, product));

}
