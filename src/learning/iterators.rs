
pub fn run (){
    let arr = [1,2,3];
    let vec = vec![12,23,32];

    for item in vec.iter() {
        println!("{}",item);
    }
    println!("{:?}", vec);

    for item in arr.into_iter() {
         println!("{}",item);
    }
    println!("{:?}", arr);
    

    // Iterator and adaptor methods

    let vec = vec![1,2,3,4,5];

    let double_vec: Vec<i32> = vec.iter().map(|x| x*2).collect();
    println!("Double Vec: {:?}",double_vec);

    // ownership of vec is transfered so vec cannot be used.
    // let double_vec_v2: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    // println!("Double Vec v2: {:?}",double_vec_v2);

    // iter() borrow the vector vec. 
    let even_vec: Vec<&i32> = vec.iter().filter(|x| **x % 2 == 0).collect();

    println!("Even {:?}",even_vec);

    // into_iter() take ownership of the vector vec. 
    let even_vec: Vec<i32> = vec.into_iter().filter(|x| *x % 2 == 0).collect();

    println!("Even v2 {:?}",even_vec);


    let vec2 = vec![1,2,3,4];

    match vec2.into_iter().reduce(|acc, item| acc +item){
        Some(sum) => println!("The sum of vector elem is : {}", sum),
        None => println!("None"),
    }


}