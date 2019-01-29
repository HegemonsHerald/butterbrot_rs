mod lib;
use lib::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::time::{Instant, Duration};

fn main() {

    /* Parse Arguments */

    let (thread_count, (sample_count, iterations, warmup), (width, height), (c1, c2), timeout) = (7, (100000, 100000, 3), (12u64, 15u64), (math::Complex::new(33.0, 42.4), math::Complex::new(9.0, 9.7897)), Duration::from_secs(100000));


    /* Do the actual thing */

    let mut birb: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::with_capacity((width * height + 2) as usize)));

    {
        let mut b = birb.lock().expect("Something went wrong with the supreme birb buffer's initial configuration!");
        b.push(width);
        b.push(height);
    }

    butterbrot_run(
        Arc::clone(&birb),
        timeout,
        thread_count,
        sample_count,
        iterations,
        warmup,
        width,
        height,
        c1,
        c2
        );


    /* Write data to file */

    // acquire the data
    // birb.into_inner()


    /* Finish properly */

    std::process::exit(0);

}
