/// Lil' helper with the errors
///
/// Use `error!( Err(42), "Oh no, It gone bad!" )` to output something like:
/// <pre><code><b style="color:red">Error:</b> Oh no, It gone bad!
/// </code></pre>
///
/// Use `error!( Err(42), "Noooooooooooooo", 1 )` (or any other valid expression in place of
/// the `1`) to output something like:
/// <pre><code><b style="color:red">Error:</b> Noooooooooooooo
///     System Error: 42
/// </code></pre>
#[macro_export]
macro_rules! error {

    ( $result:expr, $msg:expr, $format:expr ) => {

        { // I wish to run multiple statements, so I need a code block. Last statement will be return value

            $result.unwrap_or_else(|e| {
                // For some reason the adapted panic! used in the next rule doesn't work with format!,
                // so just clear the panic from any messaging
                std::panic::set_hook(Box::new(|_| { }));

                // Print custom error msg
                println!("\x1B[31;1mError:\x1B[0m {}", $msg);
                println!("\tSystem Error: {:?}", e);

                // Panic without msg
                panic!("");

            })

        }
    };

    ( $result:expr, $msg:expr ) => {

        {
            // Set custom panic message
            std::panic::set_hook(Box::new(|panic_info| {

                // Print red 'Error: ' followed by panic payload
                println!("\x1B[31;1mError:\x1B[0m {}", panic_info.payload().downcast_ref::<&str>().unwrap());

            }));

            // Panic.
            $result.unwrap_or_else(|_| {
                panic!($msg);
            })

        }
    }
}

pub mod io;
pub mod math;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::time::{Instant, Duration};

// TODO proper re-exports
// TODO make private, what can be private
// TODO replace error handling with an error handling macro!


// TODO documentation
fn write_back(orbit:&Vec<math::Complex>, supreme_birb:&mut Vec<u64>, step_size: (f64, f64)) {

    let ss_x = step_size.0;
    let ss_y = step_size.1;

    // TODO implement the thing
}

// TODO documentation
fn logging(rcv:Receiver<(i32, i32)>) {

    // TODO implement the real thing!

    loop {
        println!("{:?}", rcv.recv().unwrap());
    }

}

/// Computes the Buddahbrot Set multi-threadedly
///
/// This function looks more complicated than it is. It simply creates a number of threads,
/// runs `MHOrbits` iterators in each of them and has the threads write their computed `Orbits` to
/// `supreme_birb` -- the buffer from the main thread -- after a couple of orbits were computed.
///
/// ### This function takes a boat-load of arguments:
///
/// `supreme_birb` is an `Arc` reference to the main function's birb
///
/// `timeout` is the maximum `Duration` the computation should run, **needs** to be provided,
/// though the argument parser will set this value to u64.MAX secs, if no timeout was provided from
/// the command line
///
/// `thread_count` is the number of threads to use for computation
///
/// `sample_count` is the total number of `Orbits` to compute  
/// `iterations` is the maximum length each `Orbit` should have  
/// `warmup` is the warmup length for the `MHOrbits` iterators
///
/// `width` is the width of supreme_birb  
/// `height` is the height of supreme_birb
///
/// `corner_1` is one of the boundairy points of the frame of the complex plane, that we want to observe  
/// `corner_2` is the other point
pub fn butterbrot_run(

    supreme_birb:Arc<Mutex<Vec<u64>>>,

    timeout: Duration,

    thread_count:i32,

    sample_count:i32,
    iterations:i32,
    warmup:i32,

    width: u64,
    height: u64,

    corner_1: math::Complex,
    corner_2: math::Complex

    ) {

    /* Setup multi-threading and write_back */

    let timestamp = Instant::now();

    let step_size = (
        (corner_1.r - corner_2.r).abs() / (width  as f64),  // stepsize in x direction
        (corner_1.i - corner_2.i).abs() / (height as f64)   // stepsize in y direction
        );

    // How many orbits to compute per write_back phase in each thread
    let pl        = (width * height + 2) / thread_count as u64;
    let phase_len = if pl > 0 { pl } else { 1 };    // Integer division might yield 0

    // Note: Each thread gets to compute only so many orbits, as to only slightly exceed twice the
    // size of supreme_birb for total memory consumption

    let (log_snd, log_rcv) = channel();


    /* Make the threads */

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for thread_index in 0..thread_count {

        /* Setup clones of variables (cause move-closure) */

        let supreme = Arc::clone(&supreme_birb);

        let timestamp = timestamp.clone();
        let timeout   = timeout.clone();

        let log_snd  = log_snd.clone();


        /* Make the thread */

        let t = thread::spawn(move || {

            // Itsy-bitsy bit of logging directly from here!
            println!("Started thread {}", thread_index);

            // Create necessary data structures
            let mut orbits: Vec<Vec<math::Complex>> = Vec::with_capacity(phase_len as usize);
            let mut mh_orbits = math::MHOrbits::new(sample_count, warmup, iterations, corner_1, corner_2);

            let mut delta_t = timestamp.elapsed();

            // Compute!
            while !mh_orbits.finished() && delta_t <= timeout {

                /* Produce new orbits */

                for _ in 0..phase_len {

                    if let Some(o) = mh_orbits.next() { orbits.push(o) }
                    // not breaking on None here, cause we need logging info send to the logger fn

                }


                /* Write back to supreme birb */

                // TODO rewrite error handling
                let mut birb = supreme.lock().expect("Couldn't acquire Mutex lock");

                orbits.iter().for_each(|o| write_back(o, &mut *birb, step_size));

                orbits.clear(); // so I can reuse this on the next cycle


                /* Send logging info */

                error!(log_snd.send((thread_index, mh_orbits.remaining())), "\x1B[31;1mError:\x1B[0m Sending logging data from thread {} failed. This indicates something was wrong with the main thread!");


                /* Check the timeout */

                delta_t = timestamp.elapsed();

            }

        });

        handles.push(t);

    }

    /* Logging output */
    logging(log_rcv);

    /* Join */
    handles.into_iter().for_each(|h| h.join().expect("Thread didn't return properly!"));

}


#[cfg(test)]
mod tests {

    use super::io::*;
    use super::math::*;

    #[test]
    fn read_write() {

        /* Try and read the birb file */
        let buffer: Vec<u64> = read_birb("foo.birb");

        /* Try and write the birb file */
        write_birb("birb.birb", &buffer);

    }

    #[test]
    fn make_example_data() {

        let mut buffer:Vec<u64> = Vec::new();

        // width and height
        buffer.push(2);
        buffer.push(2);

        buffer.push(42);
        buffer.push(420);
        buffer.push(4200);
        buffer.push(42000);

        // this thing won't write somehow...
        // write_birb("data.birb", &buffer);
    }

    #[test]
    fn orbit_struct() {

        let orbit = Orbit::new(Complex{r:33.0,i:32.0}, 10);

        for i in orbit {
            println!("Complex number:\t{{ real: {}, imag: {} }}", i.r, i.i);
        }

    }

    #[test]
    fn mh_orbits_struct() {

        let mut mh_orbit = MHOrbits::new(5, 30, 4, Complex::new(-1f64, -7.0), Complex::new(10.0, 10.0));

        for i in mh_orbit {
            let mut vv: Vec<Complex>;
            println!(":=========================================:");

            for j in i {
                println!("Complex number:\t{{ real: {}, jmag: {} }}", j.r, j.i);
            }

        }

    }


}
