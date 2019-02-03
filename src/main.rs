mod lib;
use lib::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {

    /* Parse Arguments */

    let ( (width, height), (c1, c2), filename, thread_count, (sample_count, iterations, warmup), to) = io::parse_args(std::env::args().collect());
    let timeout = Duration::from_secs(to);


    /* Do the actual thing */

    let mut birb: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::with_capacity((width * height + 2) as usize)));

    {
        let mut b = error!(birb.lock(), "Something went wrong with the supreme birb buffer's initial configuration!");
        b.push(width);
        b.push(height);

        // Set all counters to 0
        (0..(width*height)).for_each(|_| { b.push(0) });
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
        c2,
        &filename
        );


    /* Write data to file */

    // acquire the data
    let birb = error!(birb.lock(), "Couldn't acquire Mutex Lock for writing the birb to a file!");

    birb.iter().enumerate().for_each(|(i,v)| println!("{} {}", i, v));



    /* Finish properly */

    std::process::exit(0);

}
