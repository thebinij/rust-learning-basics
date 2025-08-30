use rocket::*;
use csv::{Reader, Writer};
use ::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::Status;
use std::fs::File;
use std::io::Read;

// Run App using Cargo-Watch -> Cargo watch -x run

#[get("/")]
fn hello_user() -> &'static str{
    "Hello User!"
}

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_task();
    Json(tasks)
}

#[post("/create-task", format="json", data="<task>")]
fn create_task(task: Json<Task>)-> Status {

    let mut tasks = load_task();

    if let Some(_index) = tasks.iter().position(|item| item.task_name == task.0.task_name){
       return Status::Conflict;
    }
    tasks.push(task.0);
    save_tasks(&tasks);
    Status::Created

}

#[put("/update-task", format="json", data="<task>")]
fn update_task(task: Json<Task>)-> Status {

    let mut tasks = load_task();

    if let Some(index) = tasks.iter().position(|item| item.task_name == task.0.task_name){
       tasks.remove(index);
       tasks.insert(index, task.0);
       save_tasks(&tasks);
       return Status::NoContent;
    }
    else{
        Status::NotFound
    }
}


#[delete("/delete-task", format="json", data="<task>")]
fn delete_task(task: Json<Task>)-> Status {

    let mut tasks = load_task();

    if let Some(index) = tasks.iter().position(|item| item.task_name == task.0.task_name){
       tasks.remove(index);
       save_tasks(&tasks);
       return Status::NoContent;
    }
    else{
        Status::NotFound
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_user,fetch_tasks,create_task,update_task,delete_task])
}


// Task.rs

#[derive(Serialize,Deserialize, Debug)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_complete: String,
}

pub fn load_task()->Vec<Task>{
    let mut tasks = vec![];

    let mut file = File::open("tasks.csv").unwrap_or_else( |_|
        File::create("tasks.csv").unwrap()
    );
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_)=> {
            let mut reader = Reader::from_reader(contents.as_bytes());
            for result in reader.deserialize() {
                let task = result.unwrap();
                tasks.push(task);
            }
        },
        Err(_) => (),
    }

    tasks
}

pub fn save_tasks(tasks: &Vec<Task>){
    let file = File::create("tasks.csv").unwrap();

    let mut writer = Writer::from_writer(file);

    for task in tasks{
        writer.serialize(task).unwrap();
    }
}

