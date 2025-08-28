#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
}

impl Shape {
    fn new_circle(radius: f64) -> Self {
        Self::Circle(radius)
    } 

    fn new_rectange(l: f64, b: f64)-> Self{
        Self::Rectangle(l, b)
    }

    fn area(&self){
        match self{
            Shape::Circle(radius) => println!("Area of Cirlcle :{}",3.14*radius),
            Shape::Rectangle(len, bre) => println!("Area of rectangle :{}",len*bre)
        }
    }
}

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
// Option Enum example
fn get_user_phone_number(user_id: i32) -> Option<String>{
    let mob  =  "981212".to_string();
    if user_id == 1 {
       return Some(mob);
    } else {
       return None;
    }
}

fn divide(x:i32, y:i32)-> Result<i32, String>{
    if y == 0 {
        return  Err("Y is Zero".to_string());
    } else {
        return Ok(x/y);
    }
}

use std::fmt::Display;
fn print_data<T: Display>(data: T) -> T{
    println!("Data: {}", data);
    data
}

struct Circle {
    radius: f64,
}

trait TShape {
    fn area_circle(&self) -> f64;
    fn perimeter_circle(&self) -> f64;
    
}
impl TShape for Circle {
     fn area_circle(&self) -> f64{
        3.1*self.radius*self.radius
     }
     
    fn perimeter_circle(&self) -> f64 {
        todo!()
    }
}


pub fn run() {
    let rec_one = Rectangle::new(6,10);
    
    let rec_two = Rectangle::new(7,8);

    let result_one = rec_one.area();
    println!("{:?}",rec_one);
    println!("{:?}",result_one);

    let result_two =rec_two.area();
    println!("{:?}",result_two);

    let circle = Shape::new_circle(5.1);
    println!("{:?}",circle);

    let rect = Shape::new_rectange(5.1, 2.4);
    println!("{:?}",rect);

    circle.area();
    rect.area();

    match get_user_phone_number(1) {
        Some(data) => println!("Phone number: {}",data),
        None => println!("User Mobile number does not exist")
    }

    let result = divide(10,5);
    println!("Result {:?}",result);

    print_data(true);

    let new_circle = Circle {
        radius: 5.0
    };

    println!("{}",new_circle.area_circle());

    new_circle.perimeter_circle();
 
}


/*
Methods vs Associated Functions

Feature    |  Methods     | Associated Functions

Definition | Within impl, first param self | Withing impt, no self param

Access to Instance | Yes | No
Syntax for Calling instance.method() | TypeName::function()

*/ 

/*
 Traits

 Object is OOP  -> [Data, Behaviour]

 Rust -> [ Data (Enum/Structs) ] [ Behaviour (traits) ]

 */
