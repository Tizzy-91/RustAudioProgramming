use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

// scheduling and execution of code with threads
// mutexes and shared state
// message passing between threads
// shared memory concurrency
// hung up waiting for a thread to finish
// using channels to transfer data between threads

// the spawned thread stops printing after 20 times.
// my guess is the main thread for loop finishes, and 
// the main function reaches the end of its code block
// before the final sleep() finishes in the spanwed thread.
// when main thread finishes, then the program is terminated.
// fn main() {
//     // this is the creation of a thread
//     thread::spawn(|| {
//         for i in 1..25{
//             println!("spawned thread : {}", i);
//             // putting this thread to sleep gives our other thread
//             // the option to execute
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..20 {
//         println!("main thread : {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// ===================================================
// to ensure all the threads finishe, we need to call thread.join()
// fn main() {
//     // this is the creation of a thread
//     let thread1 = thread::spawn(|| {
//         for i in 1..25{
//             println!("spawned thread : {}", i);
//             // putting this thread to sleep gives our other thread
//             // the option to execute
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..20 {
//         println!("main thread : {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     // ##################### joining
//     thread1.join().unwrap();
// }


// ===================================================
// bank account example, causes compile error because the 
// spanwed thread may outlive the borrowed variable bank.
// this would be bad. there is a way to fix with smart pointers,
// which is the next example
// fn main() {

//     pub struct Bank  {
//         balance: f32
//     }

//     fn withdraw(the_bank: &mut Bank, amt: f32){
//         the_bank.balance -= amt;
//     }

//     // {curly} braces when instantiating a struct, not (brackets)
//     let mut bank: Bank = Bank {balance: 100.0};
//     withdraw(&mut bank, 7.0);
//     println!("bank balance: {}", bank.balance);

//     fn customer(the_bank: &mut Bank){
//         withdraw(the_bank, 2.00);
//     }

//     thread::spawn(|| {
//         customer(&mut bank)
//     }).join().unwrap();
// }

fn main() {

    pub struct Bank  {
        balance: f32
    }

    // Arc is a smart pointer that allows multiple owners of the same data
    // Mutex is a smart pointer that allows only one thread to access the data at a time
    fn withdraw(the_bank: Arc<Mutex<Bank>>, amt:f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < amt {
            println!("insufficient funds");
            return;
        }
        bank_ref.balance -= amt;
        println!("withdrawal of {} successful, new balance : {}", amt, bank_ref.balance);
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(the_bank, 3.50);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new
        (Mutex::new(Bank {balance: 100.0}));
    
    // || is a closure, which is a function that can be called later
    // |_| is a placeholder for a variable that will be passed in later
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("bank balance: {}", bank.lock().unwrap().balance);
}