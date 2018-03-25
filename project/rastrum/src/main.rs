use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

const NTHREADS: usize = 4; 

struct Rastrum {
    monitor_lock: Mutex<bool>,
    knigths_cvs: Vec<Condvar>,
}

impl Rastrum {
    pub fn new() -> Rastrum{
        let mut knigths_cvs = vec![];
        for _ in 0..NTHREADS {
            knigths_cvs.push(Condvar::new());
        }
        Rastrum {
            monitor_lock: Mutex::new(false),
            knigths_cvs: knigths_cvs,
        }
    }

    fn start_talking(&self, knight_id : thread::ThreadId) {
        let mutex = &self.monitor_lock;
        *mutex.lock().unwrap();
        println!("Started! id: {:?}", knight_id);
        drop(mutex);
    }

    fn stop_talking(&self, knight_id : thread::ThreadId) {
        let mutex = &self.monitor_lock;
        *mutex.lock().unwrap();
        println!("Stopped! id: {:?}", knight_id);
        drop(mutex);
    }
}

fn knight_main(r: Arc<Rastrum>) {
    let one_second = time::Duration::from_secs(1);
    let knight_id : thread::ThreadId = thread::current().id();
    r.start_talking(knight_id);
    thread::sleep(one_second);
    r.stop_talking(knight_id);
}

fn main() {
    let rastrum = Arc::new(Rastrum::new());

    let mut knights = vec![];

    for _ in 0..NTHREADS {
        let r = rastrum.clone();
        knights.push(thread::spawn(move || {
            knight_main(r);
        }));
    }

    for knight in knights {
        knight.join().expect("Knight had an error.");
    }
}
