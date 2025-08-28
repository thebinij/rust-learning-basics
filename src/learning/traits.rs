trait  Greet {
    fn greet(&self){
        println!("Hello Greet!")
    }
}

struct Person;

impl Greet for Person {
}

struct Student;

impl Greet for Student{
    fn greet(&self){
        println!("Hello Student!")
    }
}