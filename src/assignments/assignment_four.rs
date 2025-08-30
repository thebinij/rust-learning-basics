
/*
Assingment Four: what is answer for the given code.

Options:

A. It will compile and print: x+y:15
B. It will not compile due to a lifetime error
C. It will panic at runtime due to acessing a freed varible
D. It will compile, but result in undefined behaviour.
*/

pub fn run (){
    let x = 10;
    let y = get_val();

    println!("x+y:{}",x+y)
}

fn get_val() -> &i32 {
    let y = 5;
    &y
}