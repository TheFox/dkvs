
use std::time::Duration;
use std::time::Instant;

use chrono::Local;
use chrono::DateTime;

struct Task {
    name: String,
    dur: Duration,
    last_run_at: Option<DateTime<Local>>,
}

impl Task {
    fn new(name: String, dur: Duration) -> Self {
        println!("-> Task::new()");
        Self {
            name: name,
            dur: dur,
            last_run_at: None,
        }
    }

    fn run(&mut self) {
        println!("-> Task::run() -> {}", self.name);
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

    pub fn add_task(&mut self, name: String, dur: Duration) {
        println!("-> Manager::add_task({})", &name);

        let task = Task::new(name);
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        println!("-> Manager::run()");
    }
}
