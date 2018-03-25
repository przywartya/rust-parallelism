use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time};

const NTHREADS: usize = 4;

#[derive(Debug)]
enum KnightState {
    Idle,
    Talking,
    Willing,
}

struct Rastrum {
    knights_cvs: Vec<Condvar>,
    knight_states: Vec<KnightState>,
}

impl Rastrum {
    pub fn new() -> Rastrum{
        let mut knights_cvs = vec![];
        let mut knight_states = vec![];
        for _ in 1..NTHREADS+1 {
            knights_cvs.push(Condvar::new());
            knight_states.push(KnightState::Idle);
        }
        Rastrum {
            knights_cvs: knights_cvs,
            knight_states: knight_states,
        }
    }

    fn start_talking(&mut self, i : usize) {
        println!("Started! id: {:?}", i);
        self.knight_states[i%NTHREADS] = KnightState::Willing;
        println!("Knight states {:?}", self.knight_states);
        let (left, right) = (
            &self.knight_states[(i-1)%NTHREADS],
            &self.knight_states[(i+1)%NTHREADS],
        );
        println!("{:?} :: {:?}", left, right);
        // if (left != KnightState::Talking
        //     && right != KnightState::Talking)
    }

    fn stop_talking(&self, i : usize) {
        println!("Stopped! id: {:?}", i);
    }
}

fn knight_main(i: usize, r: Arc<Mutex<Rastrum>>) {
    let one_second = time::Duration::from_secs(1);

    let mut lock_start = r.lock().unwrap();
    lock_start.start_talking(i);
    drop(lock_start);

    thread::sleep(one_second);

    let lock_stop = r.lock().unwrap();
    lock_stop.stop_talking(i);
    drop(lock_stop);
}

fn main() {
    let rastrum = Arc::new(Mutex::new(Rastrum::new()));
    let mut knights = vec![];

    for i in 1..NTHREADS+1 {
        let r = rastrum.clone();
        knights.push(thread::spawn(move || {
            knight_main(i, r);
        }));
    }

    for knight in knights {
        knight.join().expect("Knight had an error.");
    }
}
