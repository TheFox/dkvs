
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

pub struct Task<F>
where F: FnMut() -> () {
    name: String,
    dur: Duration,
    func: F,

    last_run_at: Instant,
}

impl<F> Task<F>
where F: FnMut() -> () {
    fn new(name: String, dur: Duration, func: F) -> Self {
        println!("-> Task::new({})", &name);
        Self {
            name: name,
            dur: dur,
            func: func,

            last_run_at: Instant::now(),
        }
    }

    fn run(&mut self) {
        // println!("-> Task::run() -> {}", self.name);

        (self.func)();
        self.last_run_at = Instant::now();
    }
}

pub struct Manager<F>
where F: FnMut() -> () {
    tasks: Vec<Task<F>>,
    wait_dur: Duration,
    start_time: Instant,
}

impl<F> Manager<F>
where F: FnMut() -> () {
    pub fn new(wait_dur: Duration) -> Self {
        println!("-> Manager::new({:?})", wait_dur);
        Self {
            tasks: vec![],
            wait_dur: wait_dur,
            start_time: Instant::now(),
        }
    }

    pub fn add_task(&mut self, name: String, dur: Duration, func: F) {
        println!("-> Manager::add_task({})", &name);

        let task = Task::new(name, dur, func);
        self.tasks.push(task);
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn run(&mut self) {
        // println!("-> Manager::run()");

        // Check tasks.
        for task in &mut self.tasks {
            // println!("-> task: {}", task.name);

            if task.last_run_at.elapsed() >= task.dur {
                // println!("-> run task: {} ({:?})", task.name, task.last_run_at);
                task.run();
            }
        }

        // Sleep
        let sleep_dur = if self.start_time.elapsed() < self.wait_dur {
            self.wait_dur - self.start_time.elapsed()
        }
        else {
            println!("-> time elapsed: {:?}", self.wait_dur);
            Duration::from_millis(1)
        };
        sleep(sleep_dur);
    }
}
