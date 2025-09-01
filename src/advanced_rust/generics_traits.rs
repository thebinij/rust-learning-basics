

fn sum_of_elements(nums : &[i32]) -> i32{
    let mut sum = nums[0]; 
    for i in 1..nums.len(){
        sum += nums[i];
    }
    sum
}

/*  Won't compile
fn sum_of_elements_generic<T>(nums : &[T]) -> T{
    let mut sum = nums[0]; // It either do copy or move. (move can illegal since T is an immutable ref.)
    for i in 1..nums.len(){
        sum += nums[i]; // add or assign maybe be possible in all types. (like for vector types)
    }
    sum
}
*/

fn sum_of_elements_generic<T: std::ops::AddAssign + Copy>(nums : &[T]) -> T{
    let mut sum = nums[0]; // here copy trait used here
    for i in 1..nums.len(){
        sum += nums[i]; // 
    } 
    sum
}

pub fn main(){
    let my_nums = [1,2,3,4,5];
    let your_nums = [6,5,4,3,2];


    // Beginner Approach
    let mut my_sum = my_nums[0];
    for i in 1..my_nums.len(){
        my_sum += my_nums[i];
    }

    let mut your_sum = your_nums[0];
    for i in 1..your_nums.len(){
        your_sum += your_nums[i];
    }

    println!("My Sum :{}", my_sum);
    println!("Your Sum :{}", your_sum);


    // Better Approach with DRY principle
    let my_sum_1 = sum_of_elements(&my_nums);
    let your_sum_1 = sum_of_elements(&your_nums);

    println!("My Sum 1 :{}", my_sum_1);
    println!("Your Sum 1 :{}", your_sum_1);

    // will not compile
    let my_new_nums = [10.0, 20.2, 30.3];
    // let my_f32_sum = sum_of_elements(&my_new_nums);

    // Advanced Approach using Generics
    let my_f32_sum = sum_of_elements_generic::<f32>(&my_new_nums); //adding ::f32 type is optional but helpful in some cases
    
    println!("My Sum F32 :{}", my_f32_sum);


    // Vector is used generics.
    let v_i32 = vec![1,2,3,4];
    let v_f64 = vec![1_f64, 2_f64,3_f64];
    let v_i64: Vec<i64> = vec![100,200,300];


}