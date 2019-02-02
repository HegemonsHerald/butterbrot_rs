/// Lil' helper with the errors
///
/// This Macro does an orderly `panic!`, if it is provided with an Error variant of a Result or
/// yields the Ok value of the Result, by calling `unwrap_or_else()`.
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
///
/// `filename` is the filename...
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
    corner_2: math::Complex,

    filename: &str

    ) {

    /* Setup multi-threading and write_back */

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

    let thread_samples = sample_count / thread_count;

    let timestamp = Instant::now();


    /* Make the threads */

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(thread_count as usize);

    for thread_index in 0..thread_count {

        /* Setup clones of variables (cause move-closure) */

        let supreme = Arc::clone(&supreme_birb);

        let timestamp = timestamp.clone();
        let timeout   = timeout.clone();

        let log_snd  = log_snd.clone();


        /* Make the thread */

        let t = thread::spawn(move || {

            // Itsy-bitsy bit of logging directly from here!
            println!("Thread {} in WarmUp", thread_index);

            // Create necessary data structures
            let mut orbits: Vec<Vec<math::Complex>> = Vec::with_capacity(phase_len as usize);
            let mut mh_orbits = math::MHOrbits::new(thread_samples, warmup, iterations, corner_1, corner_2);

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

                thread::sleep(Duration::from_secs(1));

            }

            println!("Thread {} computed its payload", thread_index);

        });

        handles.push(t);

    }


    /* Logging output */
    let rx = logging(log_rcv, width, height, thread_count, sample_count, iterations, corner_1, corner_2, filename, timestamp, timeout);

    /* Join */
    handles.into_iter().for_each(move |h| h.join().expect("Thread didn't return properly!"));

    println!("All threads finished.");

}

// TODO documentation
fn write_back(orbit:&Vec<math::Complex>, supreme_birb:&mut Vec<u64>, step_size: (f64, f64)) {

    let ss_x = step_size.0;
    let ss_y = step_size.1;

    // TODO implement the thing
}

/// generates a String with the *unchanging* part of the logging output
fn static_msg(width:u64, height:u64, iterations:i32, sample_count:i32, c1:math::Complex, c2:math::Complex, filename:&str) -> String {

    let w = format!("width: {}", width);
    let i = format!("iterations: {}", iterations);

    // The number of spaces to insert after either 'width' or 'iterations',
    // depending on which is longer...
    let mut w_spaces = 0;
    let mut i_spaces = 0;

    let d = (w.len() as i64 - i.len() as i64).abs();

    match w.len() > i.len() {
        true  => i_spaces = d,
        false => w_spaces = d,
    }

    // The '{tab:>width$}' parts insert a right aligned tab char after width spaces

    format!("{}\n{}\n{}\n{}\n{}",
            format_args!("{w}{tab:>width$}{h}", w = w, h = format_args!("height: {}", height),        tab = "\t", width = w_spaces as usize),
            format_args!("{i}{tab:>width$}{s}", i = i, s = format_args!("samples: {}", sample_count), tab = "\t", width = i_spaces as usize),
            format_args!("complex1: {{ r: {}, i: {} }}", c1.r, c1.i),
            format_args!("complex2: {{ r: {}, i: {} }}", c2.r, c2.i),
            format_args!("filename: {}", filename),
            )

}

/// generates a String with the dynamic thread logging output
///
/// `total` should be the total number of samples *per thread*, not for the entire program run.  
/// `data` is the tuple, that comes back from a thread via the mpsc-channel.
fn thread_msg(data:(i32, i32), total:i32) -> String {

    format!("thread {} {{ done: {1:>8}, left: {2:>8}, percent: {3:>6}% done }}", data.0, total - data.1, data.1, ((total - data.1) as f32 / total as f32) * 100f32)

}

/// generates a String with the dynamic computation wide logging output
///
/// `done` should be the sum number of how many samples all thread have computed so far.  
/// `total` should be the total number of samples all threads should compute together
fn status_msg(done:i32, total:i32, timestamp:Instant, timeout:Duration) -> String {

    let a  = "samples done / total:   ";
    let b  = "percentage done:        ";
    let c  = "time elapsed:           ";
    let d  = "left / maximum runtime: ";

    let ts = timestamp.elapsed().as_secs();
    let to = timeout.as_secs();

    format!("{}{} / {}\n{}{}%\n{}{}s\n{}{}s / {}s\n",
            a, done, total,
            b, ((total - done) as f32 / total as f32) * 100f32,
            c, ts,
            d, to - ts, to)

}

// TODO documentation
// Also note that this function returns the receiver it gets passed,
// cause when a timeout is set it most definitely would finish before
// all the threads return and this way it doesn't panic, when the threads
// try to send their last piece of logging data
fn logging(
    rx:Receiver<(i32, i32)>,
    width:u64,height:u64,
    threads:i32,
    sample_count:i32,
    iterations:i32,
    c1:math::Complex,
    c2:math::Complex,
    filename:&str,
    timestamp:Instant,
    timeout:Duration) -> Receiver<(i32,i32)> {

    let mut msg:Vec<Option<(i32,i32)>> = Vec::with_capacity(threads as usize);
    for _ in 0..threads { msg.push(None) }

    let static_message = static_msg(width, height, iterations, sample_count, c1, c2, filename);
    let thread_samples = sample_count / threads;

    let mut delta_t = timestamp.elapsed();

    while delta_t <= timeout {

        // Try to get a message from the channel
        if let Ok((a,b)) = rx.try_recv() {
            msg[a as usize] = Some((a,b))
        }

        // If there are NO Nones among the messages, we can output a new log
        let b: Option<Vec<_>> = msg.iter().cloned().collect();
        if b != None {

            // Print static message
            println!("\n{}\n", static_message);

            // Print thread messages
            msg.iter()
                .cloned()
                .for_each(|a| {
                    match a {
                        Some(v) => println!("{}", thread_msg(v, thread_samples)),
                        None    => {},
                    }
                });

            // Print status message
            let done = msg.iter()
                .cloned()
                .fold(0, |acc, val| {
                    match val {
                        Some((_,v)) => acc+v,
                        None        => acc,
                    }});
            println!("\n{}", status_msg(done, sample_count, timestamp, timeout));

            // Reset msg to None values
            msg = msg.iter()
                .cloned()
                .map(|v| {
                    match v {
                        Some((a,0)) => Some((a,0)),
                        _ => None,
                    }
                }).collect();

            // Break if all threads have reached 0 samples left
            // (This changes acc whenever it encounters a pair, that hasn't reached 0, so if all
            // threads are at 0, the acc will remain 0)
            if msg.iter().cloned().fold(0i32, |acc, v| {
                match v {
                    Some((_,0)) => acc,
                    _ => acc + 1,
                }
            }) == 0 {
                break
            }

        }

        delta_t = timestamp.elapsed();

    }

    rx

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
