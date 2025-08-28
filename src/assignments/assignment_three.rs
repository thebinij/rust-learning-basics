/*
Assignment - 3 

Given the following vector of integers: let number = vec![1,2,3,4,5];

Step-by-step Goals:
1. Convert the vector into an iterator using .iter().
2. Filter only the odd numbers from the list using .filter(..).
3. Increment each odd number by 1 using .map(..).
4. Find the first item in the result that is equal to 6 using .find(...).
5. Print the final result using println!.

*/


pub fn run (){
    let numbers = vec![1,2,3,4,5];

    match numbers.iter()
        .filter(|x|*x % 2 != 0)
        .collect::<Vec<&i32>>()
        .into_iter()
        .map(|x| x +1 )
        .collect::<Vec<i32>>()
        .into_iter().find(|x| *x == 6){
        Some(number) =>  println!("Final Result: {:?}",number),
        None => println!("No Result"),
    };

    let numbers_v2 = vec![1,2,3,4,5];

    match numbers_v2.iter()
        .filter(|x| **x % 2 != 0)
        .map(|x| x + 1 )
        .find(|x| *x == 6) 
        {
        Some(number) =>  println!("Final Result v2: {:?}",number),
        None => println!("No Result"),
    };

}