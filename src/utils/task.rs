
use std::time::Duration;
use std::time::Instant;

use chrono::Local;
use chrono::DateTime;

struct Task<F>
where F: FnMut() -> () {
    name: String,
    dur: Duration,
    func: F,

    // last_run_at: Option<DateTime<Local>>,
    last_run_at: Instant,
}

impl<F> Task<F>
where F: FnMut() -> () {
    fn new(name: String, dur: Duration, func: F) -> Self {
        println!("-> Task::new({})", &name);
        // let now = ;
        Self {
            name: name,
            dur: dur,
            func: func,

            last_run_at: Instant::now(),
        }
    }

    fn run(&mut self) {
        println!("-> Task::run() -> {}", self.name);

        (self.func)();
        self.last_run_at = Instant::now();
    }
}

pub struct Manager<F>
where F: FnMut() -> () {
    tasks: Vec<Task<F>>,
}

impl<F> Manager<F>
where F: FnMut() -> () {
    pub fn new() -> Self {
        println!("-> Manager::new()");
        Self {
            tasks: vec![],
        }
    }

    pub fn add_task(&mut self, name: String, dur: Duration, func: F) {
        println!("-> Manager::add_task({})", &name);

        let task = Task::new(name, dur, func);
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        // println!("-> Manager::run()");

        for task in &mut self.tasks {
            // println!("-> task: {}", task.name);

            if task.last_run_at.elapsed() >= task.dur {
                println!("-> run task: {}", task.name);
                task.run();
            }
        }
    }
}
