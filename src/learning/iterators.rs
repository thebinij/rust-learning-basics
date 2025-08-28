
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
}