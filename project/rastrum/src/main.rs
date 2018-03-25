use std::sync::{Arc};
use std::{thread, time};
const NTHREADS: usize = 4; 

struct Rastrum {

}

impl Rastrum {
    fn start_talking(&self) {
        println!("Started!");
    }

    fn stop_talking(&self) {
        println!("Stopped!");
    }
}

fn main() {
    let rastrum = Arc::new(Rastrum {});
    let one_second = time::Duration::from_secs(1);
    
    let mut knights = vec![];

    for _ in 0..NTHREADS {
        let r = rastrum.clone();
        knights.push(thread::spawn(move || {
            println!("Knight id: {:?}", thread::current().id());
            r.start_talking();
            thread::sleep(one_second);
            r.stop_talking();
        }));
    }

    for knight in knights {
        knight.join().expect("Knight had an error.");
    }
}
