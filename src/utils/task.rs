
use std::time::Duration;
use std::time::Instant;

use chrono::Local;
use chrono::DateTime;

type TaskFn = fn() -> ();

struct Task {
    name: String,
    dur: Duration,
    func: Box<Fn()>,
    last_run_at: Option<DateTime<Local>>,
}

impl Task {
    fn new(name: String, dur: Duration, func: Box<Fn()>) -> Self {
        println!("-> Task::new({})", &name);
        Self {
            name: name,
            dur: dur,
            func: func,
            last_run_at: None,
        }
    }

    fn run(&mut self) {
        println!("-> Task::run() -> {}", self.name);
        (self.func)();
    }
}

pub struct Manager {
    tasks: Vec<Task>,
}

impl Manager {
    pub fn new() -> Self {
        println!("-> Manager::new()");
        Self {
            tasks: vec![],
        }
    }

    pub fn add_task(&mut self, name: String, dur: Duration, func: Box<Fn()>) {
        println!("-> Manager::add_task({})", &name);

        let task = Task::new(name, dur, func);
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        println!("-> Manager::run()");

        for task in &self.tasks {
            println!("-> task: {}", task.name);
        }
    }
}
