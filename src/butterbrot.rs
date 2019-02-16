mod lib;
use lib::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {

    /* Parse Arguments and setup */

    let ( (width, height), (c1, c2), filename, thread_count, (sample_count, iterations, warmup, phase_len), (to,int)) = butterbrot::parse_args(std::env::args().collect());

    let timeout          = Duration::from_secs(to);
    let logging_interval = Duration::from_secs(int);

    // Used to compute the 'total' time taken right at the end
    let outer_timestamp = Instant::now();


    /* Do the actual thing */

    let birb: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::with_capacity((width * height + 2) as usize)));

    {
        let mut b = error!(birb.lock(), "Something went wrong with the supreme birb buffer's initial configuration!");

        // Set all counters to 0
        (0..(width*height + 2)).for_each(|_| { b.push(0) });

        b[0] = width;
        b[1] = height;

    }

    butterbrot_run(
        Arc::clone(&birb),
        timeout,
        logging_interval,
        thread_count,
        sample_count,
        iterations,
        phase_len,
        warmup,
        width,
        height,
        c1,
        c2,
        &filename
        );


    /* Write data to file */

    let birb = error!(birb.lock(), "Couldn't acquire Mutex Lock for writing the birb to a file!");

    println!("\nNow writing to file {b}{}{w}", filename, b = "\x1B[34m", w = "\x1B[0m");
    io::write_birb(&filename, &birb);
    println!("{g}Successfully wrote to file.{w}", g = "\x1B[32m", w = "\x1B[0m");

    println!("Total time taken: {g}{}s{w}", outer_timestamp.elapsed().as_secs(), g = "\x1B[32m", w = "\x1B[0m");


    /* Finish properly */

    std::process::exit(0);

}
