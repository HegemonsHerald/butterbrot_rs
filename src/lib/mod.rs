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
                eprintln!("\x1B[31;1mError:\x1B[0m {}", $msg);
                eprintln!("\tSystem Error: {:?}", e);

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
                eprintln!("\x1B[31;1mError:\x1B[0m {}", panic_info.payload().downcast_ref::<&str>().unwrap());

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
            let white  = "\x1B[0m";
            let red    = "\x1B[31m";
            let yellow = "\x1B[33m";
            println!("Thread {r}{}{w} in WarmUp", thread_index, r=red, w=white);

            // Create necessary data structures
            let mut orbits: Vec<Vec<math::Complex>> = Vec::with_capacity(phase_len as usize);
            let mut mh_orbits = math::MHOrbits::new(thread_samples, warmup, iterations, corner_1, corner_2);

            println!("{y}Thread {r}{}{y} now computing payload{w}", thread_index, y=yellow, r=red, w=white);

            let mut delta_t = timestamp.elapsed();

            // Compute!
            while !mh_orbits.finished() && delta_t <= timeout {

                /* Produce new orbits */

                for _ in 0..phase_len {

                    if let Some(o) = mh_orbits.next() { orbits.push(o) }
                    // not breaking on None here, cause we need logging info send to the logger fn

                }


                /* Write back to supreme birb */

                let mut birb = error!(supreme.lock(), "Couldn't acquire Mutex lock");

                orbits.iter().for_each(|o| write_back(o, &mut *birb, step_size));

                orbits.clear(); // so I can reuse this on the next cycle


                /* Send logging info */

                error!(log_snd.send((thread_index, mh_orbits.remaining())), "\x1B[31;1mError:\x1B[0m Sending logging data from thread {} failed. This indicates something was wrong with the main thread!");


                /* Check the timeout */

                delta_t = timestamp.elapsed();

                // TODO remove this line for PRODUCTION
                // thread::sleep(Duration::from_secs(1));

            }

            // Itsy-bitsy output on success
            let white = "\x1B[0m";
            let green = "\x1B[32m";
            println!("{g}Thread {} computed its payload{w}", thread_index, w=white, g=green);

        });

        handles.push(t);

    }


    /* Logging output */
    let rx = logging(log_rcv, width, height, thread_count, sample_count, iterations, corner_1, corner_2, filename, timestamp, timeout);

    /* Join */
    handles.into_iter().for_each(move |h| h.join().expect("Thread didn't return properly!"));


    let white = "\x1B[0m";
    let green = "\x1B[32m";
    println!("{g}All threads finished.{w}", w=white, g=green);

}

// TODO documentation
fn write_back(orbit:&Vec<math::Complex>, supreme_birb:&mut Vec<u64>, step_size: (f64, f64)) {

    let ss_x = step_size.0;
    let ss_y = step_size.1;

    // TODO implement the thing
}

/// generates a String with the *unchanging* part of the logging output
fn static_msg(width:u64, height:u64, iterations:i32, sample_count:i32, c1:math::Complex, c2:math::Complex, filename:&str) -> String {

    let white  = "\x1B[0m";
    let yellow = "\x1B[33m";
    let blue   = "\x1B[34m";

    let w = format!("width: {y}{0}{w}", width,           w=white, y=yellow);
    let i = format!("iterations: {y}{0}{w}", iterations, w=white, y=yellow);

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
            format_args!("{w}{tab:>width$}{h}", w = w, h = format_args!("height: {y}{0}{w}", height,        w=white, y=yellow), tab = "\t", width = w_spaces as usize),
            format_args!("{i}{tab:>width$}{s}", i = i, s = format_args!("samples: {y}{0}{w}", sample_count, w=white, y=yellow), tab = "\t", width = i_spaces as usize),
            format_args!("complex1: {{ r: {y}{}{w}, i: {y}{} {w}}}", c1.r, c1.i, w=white, y=yellow),
            format_args!("complex2: {{ r: {y}{}{w}, i: {y}{} {w}}}", c2.r, c2.i, w=white, y=yellow),
            format_args!("filename: {b}{}{w}", filename, w=white, b=blue),)

}

/// generates a String with the dynamic thread logging output
///
/// `total` should be the total number of samples *per thread*, not for the entire program run.  
/// `data` is the tuple, that comes back from a thread via the mpsc-channel.
fn thread_msg(data:(i32, i32), total:i32) -> String {

    let white  = "\x1B[0m";
    let red    = "\x1B[31m";
    let yellow = "\x1B[33m";

    format!("thread {r}{} {w}{{ done: {y}{1:>8}{w}, left: {y}{2:>8}{w}, percent: {y}{3:>6}% {w}done }}", data.0, total - data.1, data.1, ((total - data.1) as f32 / total as f32) * 100f32, w=white, y=yellow, r=red)

}

/// generates a String with the dynamic computation wide logging output
///
/// `done` should be the sum number of how many samples all thread have computed so far.  
/// `total` should be the total number of samples all threads should compute together
fn status_msg(done:i32, total:i32, timestamp:Instant, timeout:Duration) -> String {

    let white  = "\x1B[0m";
    let yellow = "\x1B[33m";

    let a  = "samples done / total:   ";
    let b  = "percentage done:        ";
    let c  = "time elapsed:           ";
    let d  = "left / maximum runtime: ";

    let ts = timestamp.elapsed().as_secs();
    let to = timeout.as_secs();

    format!("{}{y}{} {w}/{y} {}{w}\n{}{y}{}%{w}\n{}{y}{}s{w}\n{}{y}{}s {w}/{y} {}s{w}\n",
            a, done, total,
            b, (done as f32 / total as f32) * 100f32,
            c, ts,
            d, to - ts, to,
            w=white, y=yellow)

}

/// Outputs logging information about the state of the threads handled by `butterbrot_run`
///
/// ### What this does
/// This function sporadically composes and prints a log with the computation's state based on
/// status info it gets from the computation threads via an `mpsc` channel. The computation threads
/// send a pair with their unique index and the number of samples, their iterator has left to do.
/// Whenever `logging()` has gotten such a pair for each of the threads, it will output a log
/// message. It always waits, til it has received at least one message from all threads, so the log
/// will be complete.  
/// If the threads are enough out of sync, that a thread sends multiple messages, while another
/// hasn't send any, only the newest message will be kept.
///
/// There are some special behaviours to keep in mind. `logging()` returns when all threads have
/// finished, that is, have 0 samples left to compute.  
/// `logging()` returns an `mpsc::Receiver<(i32,i32)>`. There's a good reason for that, if a
/// custom timeout has been specified and the timeout is reached, `logging()` will almost certainly
/// return quite a while *before* the computation threads notice, that the timeout has been
/// reached. After all the computation threads will finish their current cycle, before re-checking,
/// so that, even though the timeout generally is respected, no data can be lost or mal-formed.
/// That also means, that the threads will attempt to send their last pair of logging data after
/// `logging()` already died. In order not to get an unecessary `panic!` from that, `logging()`
/// simply returns its end of the channel and expects the calling thread (main) to keep that
/// `Receiver` alive long enough, for the computation threads to `join()`.
///
/// ### Parameters
/// There's really nothing much to explain there. `timestamp` should be the `Instant` when the
/// computation was started, so the `timestamp` variable made by `butterbrot_run` right before it
/// starts creating threads. All the others are pretty obviously named...
///
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
        while let Ok((a,b)) = rx.try_recv() {
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


            /*
             * I'm only half-happy with the way the next three statements work... I do use quite a
             * lot of iterators, that are folded up or joined up or whatnot and most importantly,
             * all of them use `cloned()`, which is quite ... ehm ... MEH! ..., to figure out basic
             * info. I believe That could be done a little more elegantly, but with compiler
             * optimization and (*mostly*) because of the laziness of the iterators, this shouldn't
             * really matter much either way...
             */

            // Print status message
            let left = msg.iter()
                .cloned()
                .fold(0, |acc, val| {
                    match val {
                        Some((_,v)) => acc+v,
                        None        => acc,
                    }});
            println!("\n{}", status_msg(sample_count - left, sample_count, timestamp, timeout));

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

        // Don't hog the CPU... too much
        thread::sleep(Duration::from_millis(200)); // TODO for proper computation this can be a much longer time to wait!

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
