use std::vec;
use chrono::{Utc, DateTime, Local};

fn main() {
    println!("Hello, world!");
    let mut root_task: Task = Task {
        name: String::from("Root task"),
        description: String::from("The root task, the task which every other task comes from."),
        creation_time: get_current_time(),
        est_time: -1,
        completion_time: 0,
        completion_status: Completion::Incomplete,
        repetition_status: String::from("Non-repeating"),
        location: String::from("None"),
        sub_tasks: Vec::new(),
        estimated_priority: -1
    };
}

enum Completion {
    Incomplete,
    Complete,
    Cancelled
}

struct Task {
    name: String, //Informal name for this task.
    description: String, //Description of this task.
    creation_time: i64, //The time this task was created.
    est_time: i64, //Estimated time it will take to complete this task, not including the sub-tasks, which can be calculated upon demand.
    completion_time: i64, //The time this task was finished, if it hasn't been finished, then it'll be -1.
    completion_status: Completion, //Denotes if the task is complete.
    repetition_status: String, //Denotes if and how this task will be repeated, a `String` for now, but it might be something later.
    location: String, //Can be latitude & longitude, or an address, and also there's transportation-based-tasks, and altitude, and probably more I can't think of.
    sub_tasks: Vec<Task>, //Tasks that have to be not `Completion::Incomplete` in order for this task to be completed.
    estimated_priority: i64 //Integered index or float from 0-1, not quite sure which (For clarification on this attribute: it would be useful to not have to do a bunch of relatively unimportant things, so I'll have a comparison quiz thingy, where 2 tasks are selected and the user is asked to select which is more important, which should create a nice general priority order without having to do it manually, or worse, have to do the quiz with one question for eveyr combination of every task. Maybe I could use AI or something to fill in the blanks, maybe I'll find some other way to do it. Maybe I'll have it be estimated from what any task is a sub-task of. I'd need a pretty good algorithm for that though, don't want super important stuff being ignored because it only comes up once, because of how general it is, like safety being important but not seeing important because it's just a sub-task of one task, when really safety is probably the most important thing in tn the entire system.)
}

fn get_current_time() -> i64 {
    Utc::now().with_timezone(&Local).timestamp()
}