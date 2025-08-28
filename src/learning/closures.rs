/*
In Rust, closures are anonymous functions that can capture variables from their surrounding scope. Closures can be stored in variables, passed as arguments, or return from functions.

*/

pub fn run (){
    let add_one = |x: i32|x + 1;

    println!("{}", add_one(10));

    let mut counter = 0;

    let mut increase_counter = || {
        counter += 1;
        println!("{}",counter);
    };

    increase_counter(); // 1 
    increase_counter(); // 2
    increase_counter(); // 3

    let x: String = String::from("hello");
    let consume_and_return = || x;

    let y = consume_and_return(); // ownership transter to y
    println!("{}",y);

}