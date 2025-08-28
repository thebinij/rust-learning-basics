/*
Assignment: Implement a Custom Iterator in Rust

Problem Statement

You are required to implement a simple counter using Rust that behaves like an iterator. Your task is to define a Counter struct and implement the Iterator trait for it so that it returns the numbers from 1 to 5, one by one. 

Requirements

1. Create a struct named Counter with a single field count of type u32.
2. Implement a method new() that initalize Counter with count = 0.
3. Implement the Iterator trait for Counter:
    - Set the associated Item type to u32.
    - Implement the next() method to:
        - Increment count each time it is called.
        - Return Some(count) while count <= 5.
        - Return None once the counter exceeds 5.
    In main function, use the iterator to print numbers 1 through 5 to the console.
*/

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self{
        Self {
            count:0
        }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <5 {
            self.count += 1;
            return Some(self.count);
        }
        else{
            return None;
        }
    }
}

pub fn run(){

    let count = Counter::new();

    for value in count{
        println!("Count: {}",value);
    }

   let mut  count_v2 = Counter::new();

    println!("Count v2: {:?}",count_v2.next());
    println!("Count v2: {:?}",count_v2.next());


}