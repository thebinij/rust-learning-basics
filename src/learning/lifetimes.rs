
/*
Lifetime - Three Golden Points

The primary objective of lifetimes in Rust is to avoid dangling references.

Rust reject code that could lead to memory safely issues.

When there is a reference, there may be a chance of a dangling reference - and Rust uses lifetimes to prevent that.

Dangling Reference
 - Occurs when a program tries to access data that has already been deallocated or gone out of scope.

'Static Reference
'Static means: "This thing lives forever - as long as the program runs. "So, if something has a 'static lifetime, it will never be removed from memory until the program ends.

Ex: let x: &'static str = "hello"; 
    let x: &str = "hello";

This string is not created at runtime. it is part of program binary.
*/

struct Path<'a> {
    point_x: &'a i32,
    point_y: &'a i32,
}


pub fn run(){
    let r;            // ----------------+-- 'a
    {                       //                 |
        let x = 5;     // ---+-- 'b       |
                            //    |            |
        r = &x;             //    |            |
        println!("{}",r)    //    |            |
    }                       // ---+            |
    // println!("{}",r);    // ----------------+  x` does not live long enough


    let ( four , nine ) = (4,9);

    print_refs(&four, &nine); // four and nine are borrowed values. so lifetime of four and nine must be longer than that of print_refs.


    let result: &i32;
    let x = 20;
    let y = 10;

    result = larger(&x, &y);
    println!("{}",result);


    let p_x = 10;
    let p_y = 20;
    let game = Path { point_x: &p_x, point_y: &p_y};
    println!("x = {}, y = {}", game.point_x, game.point_y)

}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("x is {} and y is {}", x , y);
}

// here, using 'a it says to compiler that lifetime of m and n both same. so, both live long enough.
fn larger<'a>(m: &'a i32, n: &'a i32) -> &'a i32 {
    if m > n { m } else { n }
}

// lifetime_of_reference <= lifetime_of_data_it_points

/*
Lifetimes vs Scope

Concept  | Applies to
Scope    | A variable or reference
Lifetime | A Reference

*/

// Explicit Annotations
// foo<`a> - `foo` has a lifetime parameter '`a`

// foo<`a,`b> - 'foo' has a lifetime paramater either `a and `b.

