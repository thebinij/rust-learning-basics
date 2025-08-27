#[derive(Debug)]
struct Rectangle{
    length: u8,
    breadth: u8
}

impl Rectangle {
    // associative functions
    fn new(length:u8, breadth:u8)-> Self{
        Self {
            length,
            breadth
        }
    }
    // methods
    fn area(&self)-> u8{
        self.length * self.breadth
    }
}

fn main() {
    let rec_one = Rectangle::new(6,10);
    
    let rec_two = Rectangle::new(7,8);

    let result_one = rec_one.area();
    println!("{:?}",rec_one);
    println!("{:?}",result_one);

    let result_two =rec_two.area();
    println!("{:?}",result_two);
}
