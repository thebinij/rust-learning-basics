use std::{thread, sync::Arc};

use std::sync::{Mutex, RwLock};

pub fn main(){
    // let data = vec![1,2,3,4,5];

    //Arc -> It's shared pointers. it has overhead which counter how many place the data is used.
    let data = Arc::new(vec![1,2,3,4,5]);
    let mut handles = vec![];

    for i in 0..=3{
         let data_clone = Arc::clone(&data);

        //  thread need data to be move. so thread takes the owership. 
        let handle = thread::spawn(move || {
            println!("Print data in thread {}: {:?}",i, data_clone);
        }); 
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Print data in main: {:?}", data);

    // ARC alone cannot mutate data. we have to use Mutex.

    let data_1 = Arc::new(Mutex::new(vec![1,2,3,4,5]));
    let mut handles_1 = vec![];
    
    for i in 0..=3 {
        let data_clone = Arc::clone(&data_1);

        let handle_1 = thread::spawn(move ||{
            let mut data = data_clone.lock().unwrap();
            data.push(i);
        });
        handles_1.push(handle_1);
    }

    for handle in handles_1 {
        handle.join().unwrap();
    }

    println!("Print data in main: {:?}", data_1.lock().unwrap());

    // Mutex locks the resource. But if you want to only aquire lock for read only or write only we have to use RwLock


    let data_2 = Arc::new(RwLock::new(vec![6,7,8,9]));
    let mut handles_2 = vec![];

    for i in 0..=3 {
        let data_clone = Arc::clone(&data_2);

        let handle = thread::spawn(move || {
            let mut data = data_clone.write().unwrap();
            data.push(i);
        });

        handles_2.push(handle);        
    }

    for handle in handles_2 {
        handle.join().unwrap();
    }

    println!("Print data in main: {:?}", data_2.read().unwrap());
}