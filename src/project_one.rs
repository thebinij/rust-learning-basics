/*
Assignment - 1

You are building an online learning platform to manage different types of courses. You want to implement a system that can keep track of various courses and provide brief overviews of them.

Task
1. Define a trait name Course, which should have a method get_overview that returns a brief overview of the course as a string.

2. Create two structs:
    - Workshop with fields: title, instructor, and duration (in hours)
    - Seminar with fields: title, speaker, and location.

3. Implement the Course trait for both Workshop and Seminar, providing a suitable overview in each case.

4. Create function print_course_overview that takes a generic parameter of any type that implements the Course trait and prints the overview.
*/


trait Course {
    fn get_overview(&self) -> String;
}

fn print_course_overview<T: Course>(course: T) {
   println!("{}", course.get_overview());
}


struct Workshop {
    title: String,
    instructor: String,
    duration: i32,
}

struct Seminar {
    title: String,
    speaker: String,
    location: String
}

impl Course for Workshop {
    fn get_overview(&self) -> String {
       format!("Workship title: {} Instructor: {} Duration: {} hour(s)", self.title, self.instructor, self.duration)
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!("Seminar title: {} Speaker: {} Location: {}", self.title, self.speaker, self.location)
    }
}

pub fn run() {
    let workshop = Workshop{
        title: "Agentic AI".to_owned(),
        instructor: "Jack Smith".to_owned(),
        duration: 30,
    };

    let seminar = Seminar {
        title: "Crypto Currency".to_owned(),
        speaker: "Bill Tyson".to_owned(),
        location: "New Hampshire".to_owned(),
    };

    print_course_overview(workshop);

    print_course_overview(seminar);
}