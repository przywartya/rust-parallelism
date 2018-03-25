use std::sync::{Arc};
use std::{thread, time};

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

    rastrum.start_talking();
    thread::sleep(one_second);
    rastrum.stop_talking();

}
