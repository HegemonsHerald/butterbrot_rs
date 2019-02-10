extern crate rand;

use std::io::*;
use std::fs::File;
use super::math::Complex;

/// Macro to help with parsing command line arguments.
///
/// ### What does this do?
///
/// This Macro has four rules:
///
/// 1. Complex Numbers: `parse!("-c1", args, complex)`
/// 2. Strings: `parse!("--filename", args, string)`
/// 3. Primitives: `parse!("--width", args, u64)`
/// 4. Parse Errors: `parse!(unmatched, error)`
///
/// The three first variants take a token `&str`, which should be the name of the command line
/// flag, that is currently being parsed. (For error output)
///
/// The second argument is an Iterator, that produces the arguments in order, so the Macro has
/// access to the Flag's arguments. **This must be an Iterator over Strings!**
///
/// The third argument specifies the operating mode.
///
/// If it's `complex` the Macro will attempt to get the next two items from the `args` iterator and
/// parse them as `f64`.
///
/// If it's `string` the Macro will attempt to get the next item from iterator and simply yields it
/// as a `String`.
///
/// If it's some primitive type, the Macro will attempt to get the next item from the iterator and
/// parse it as the specified primitive type using `&str.parse::<TYPE>()`.
///
/// If any of these operations fail a custom error message is displayed and the application
/// **panics**.
///
/// The fourth variant is especially for dealing with unmatched command line flags. See the example
/// below.
///
/// ### Usage Example (Why should I care?)
///
/// **Note: I didn't actually test these code snippets.**  
/// **Note:* The `math::Complex` type is part of *my* `butterbrot_rs` crate, for which I made this
/// Macro.
///
/// ```
/// let args_v: Vec<String> = std::env::args().collect();
/// let args = args_v.into_iter();
///
/// // output is for config:
/// // (width, height, math::Complex( real, imaginairy ), filename)
/// let mut output = (0i32, 0i32, math::Complex::new(88.9, 72.4), "file.name".to_string());
///
/// let mut curr:String;
/// loop {
///
///     match args.next() {
///         Some(s) => curr = s,
///         None    => break,
///     }
///
///     match curr.as_ref() {
///
///         "-w" || "--width"  => { output.0 = parse!("--width",    args, i32) },
///         "-h" || "--height" => { output.1 = parse!("--height",   args, i32) },
///         "--filename"       => { output.3 = parse!("--filename", args, string) },
///         "--complex"        => { output.2 = parse!("--complex",  args, complex) },
///
///         // print an error with the flag the parsing failed on
///         unmatched_flag => { parse!(unmatched_flag, error) },
///
///     }
///
/// }
/// ```
macro_rules! parse {

    // Parse Complex Number
    ($token:expr, $source:ident, complex) => {
        {

            // Override panic! message
            std::panic::set_hook(Box::new(|_| {}));

            // Get real part of the complex number
            let r = $source.next().unwrap_or_else(|| {
                eprintln!("\x1B[31;1mError:\x1B[0m The flag '{}' expects two arguments, a real part and an imaginairy part.", $token);
                panic!("");
            });

            // Get real part of the complex number
            let i = $source.next().unwrap_or_else(|| {
                eprintln!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one more argument, the imaginairy part.", $token);
                panic!("");
            });

            // Parse the next token
            let rp = r.parse::<f64>().unwrap_or_else(|_| {
                eprintln!("\x1B[31;1mError:\x1B[0m Couldn't parse the real part '{real}' as f64 in '{token} {real} {imag}'.", real = r, imag = i, token = $token);
                panic!("");
            });

            // Parse the next token
            let ip = i.parse::<f64>().unwrap_or_else(|_| {
                eprintln!("\x1B[31;1mError:\x1B[0m Couldn't parse the imaginairy part '{imag}' as f64 in '{token} {real} {imag}'.", real = r, imag = i, token = $token);
                panic!("");
            });

            super::math::Complex::new(rp, ip)

        }
    };

    // Parse Flag with stringy argument
    ($token:expr, $source:ident, string) => {
        {

            // Override panic! message
            std::panic::set_hook(Box::new(|_| {}));

            // Get the next token, which is a String
            $source.next().unwrap_or_else(|| {
                eprintln!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one argument.", $token);
                panic!("");
            })
        }

    };

    // Parse Flag with one argument to type
    ($token:expr, $source:ident, $type:ty) => {
        {

            // Override panic! message
            std::panic::set_hook(Box::new(|_| {}));

            // Get the next token
            let t = $source.next().unwrap_or_else(|| {
                eprintln!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one argument.", $token);
                panic!("");
            });

            // Parse the next token
            let i = t.parse::<$type>().unwrap_or_else(|_| {
                eprintln!("\x1B[31;1mError:\x1B[0m Couldn't parse '{arg}' as {type} in '{token} {arg}'.", arg = t, token = $token, type = stringify!($type));
                panic!("");
            });

            i
        }
    };

    // Better error handling, than using error!
    ($msg:expr, error) => {
        {

            // Override panic! message
            std::panic::set_hook(Box::new(|_| {}));

            // Print message and panic
            eprintln!("\x1B[31;1mError:\x1B[0m Encountered invalid flag: '{}'", $msg);
            panic!("")

        }
    }
}

/// parses command line arguments
///
/// This function has a hard-coded set of CLI-args it knows. It will linearly iterate over the list
/// of user-provided arguments and parse them, as it goes along. It will gracefully `panic!` with a
/// customised and even somewhat helpful error message, thanks to the `parse!` macro.
///
/// There's one curious thing to consider: If a flag takes multiple arguments and the user didn't
/// provide enough, but instead wrote another flag, this function will treat that flag as the
/// argument to the previous flag and (*probably*) get a parse error there. That means one has to
/// be slightly careful with flags and their order, and if parameters aren't provided, this ain't
/// gonna fly... The `parse!` macro will output, that an argument is missing, if the flag is at the
/// end of the list.
///
/// This function returns a tuple with this structure:
/// ```
/// (   (width, height),
///     (complex1, complex2),
///     filename,
///     thread count,
///     (samples, iterations, warmup, phase_len),
///     (timeout, loggin_interval)
/// )
/// ```
/// For what these mean, see the helptext and the docs of `butterbrot_run` (which takes most of
/// these numbers as arguments).
///
/// If a flag isn't provided, default values are used in its place.
///
///
/// **Note:** This function is written specifically for this project, but it can easily be adapted
/// for other projects, that only require primitive command-line facilities. Simply overwrite the
/// type declaration and the defaults for the `output` variable, then use the type declaration of
/// output as the return value of the function. Lastly change the `match` against the flags to suit
/// the project's needs and adapt the `parse!` macro to reflect the new parsing needs (that
/// shouldn't be hard: take out the bit about `math::Complex`, and add whatever rules and variants
/// of rules you need and you're good to go), and this should be fine. Piece of cake.
pub fn parse_args(args_v:Vec<String>) -> ( (u64,u64), (Complex,Complex), String, i32, (i32,i32,i32,i32), (u64,u64) ) {

    let mut args = args_v.into_iter();
    args.next();    // skip the name of the application

    let mut zoom = 100f64;
    let mut center = Complex::new(0f64, 0f64);

    // TODO make the defaults and the help text align
    // output has the format:
    // ( (width, height), (c1, c2), filename, thread_count, (samples, iterations, warmup, phase_len), (timeout, logging_interval) )
    let mut output: ( (u64,u64), (Complex,Complex), String, i32, (i32,i32,i32,i32), (u64,u64) ) = (
        (400, 400),
        (Complex::new(42.0, 42.0), Complex::new(42.0, 42.0)),
        gen_filename(),
        7,
        (400, 10, 100, 10_000),
        (std::u64::MAX,10),
        );

    let mut next;

    loop {

        // Get the next flag
        match args.next() {
            Some(s) => next = s,
            None    => break,
        }

        // Parse the flag and its arguments
        match next.as_ref() {

            "--width"      | "-w"   => { (output.0).0 = parse!("--width",      args, u64)     },
            "--height"     | "-h"   => { (output.0).1 = parse!("--height",     args, u64)     },
            "--zoom"       | "-z"   => {  zoom        = parse!("--zoom",       args, f64)     },
            "--warmup"     | "-wu"  => { (output.4).2 = parse!("--warmup",     args, i32)     },
            "--timeout"    | "-to"  => { (output.5).0 = parse!("--timeout",    args, u64)     },
            "--interval"   | "-int" => { (output.5).1 = parse!("--interval",   args, u64)     },
            "--filename"   | "-o"   => {  output.2    = parse!("--filename",   args, string)  },
            "--threads"    | "-t"   => {  output.3    = parse!("--threads",    args, i32)     },
            "--samples"    | "-s"   => { (output.4).0 = parse!("--samples",    args, i32)     },
            "--complex1"   | "-c1"  => { (output.1).0 = parse!("--complex1",   args, complex) },
            "--complex2"   | "-c2"  => { (output.1).1 = parse!("--complex2",   args, complex) },
            "--center"     | "-c"   => {  center      = parse!("--center",     args, complex) },
            "--phase_len"  | "-p"   => { (output.4).3 = parse!("--phase_len",  args, i32)     },
            "--iterations" | "-i"   => { (output.4).1 = parse!("--iterations", args, i32)     },
            "--help"       | "h"    => {
                println!("USAGE:\n\n  butterbrot [ARGUMENTS]\n\n\nPOSSIBLE FLAGS AND WHAT THEY MEAN:\n\n  h, --help\n        Display this help text.\n\n  -o, --filename <filename>\n        The filename to write the computed data to. This will be a birb file.\n\n        Default: birb_{{rand}}.birb, where {{rand}} will be turned into a random\n        string, to insure the file is available.\n\n  -t, --threads <number>\n        How many threads to use for parallel computation. Note, that this is the\n        number of computation threads. The total number of threads is one\n        larger, as this doesn't include the main thread.  This works better, if\n        the total number of threads doesn't exceed the number of available\n        cores.\n\n        Default: 7\n\n  -to, --timeout <seconds>\n        How many whole seconds to run AT MINIMUM, before the program terminates\n        the computation. Note, that the program will finish some time after the\n        timeout has been reached, as each thread will finish the currently\n        active computation before returning.\n        If no timeout is specified this value will be set to the larges possible\n        unsigned 64-Bit integer, a number of seconds, that is unlikely to be\n        reached, while computation is active.\n\n  -int, --interval <seconds>\n        The logging function will attempt to output a log only after <seconds>\n        seconds have elapsed.\n\n        Default: 10\n\n  -w, --width <number>\n        How wide to make the birb.\n\n        Default: 400\n\n  -h, --height <number>\n        How tall to make the birb.\n\n        Default: 400\n\n  -z, --zoom <number>\n        How much to zoom in.\n        This zoom factor is used to map the --width and the --height onto\n        the Complex plane, relative to the complex number specified using\n        --center.\n\n        This flag is overridden by either of --complex1 and --complex2.\n\n        Using the --zoom and --center flags to control the image is more\n        convenient, than using --complex1 and --complex2 directly, since the\n        zoom method preserves the image ratio.\n\n        The <number> may be a float.\n\n        Default: 100\n\n  -c, --center <real> <imaginairy>\n        The complex number, that should be in the center point of the final\n        image.\n\n        Default: 0 0\n\n  -wu, --warmup <number>\n        How many samples should the Metropolis-Hastings Iterators discard as\n        warmup. See documentation for more.\n\n        Default: 1000\n\n  -s, --samples <number>\n        How many samples should the program compute in total, across all\n        threads. This does not include the warmup.\n\n        Default: 10000\n\n  -i, --iterations <number>\n        How many iterations long should each Orbit be at max. See documentation\n        for more.\n\n        Default: 100\n\n  -p, --phase_len <number>\n        How many Metropolis Hastings Orbits each thread computes before calling\n        write_back -- The length of a write_back phase.\n\n        Default: 10000\n\n  -c1, --complex1 <real> <imaginairy>\n        One of the corners of the frame of the Complex Plane that is to be\n        explored. This must be a diagonally opposite corner to --complex1.\n        The real and imaginairy parts must be floats.\n\n        Default: 42.0 42.0\n\n  -c2, --complex2 <real> <imaginairy>\n        One of the corners of the frame of the Complex Plane that is to be\n        explored. This must be a diagonally opposite corner to --complex1.\n        The real and imaginairy parts must be floats.\n\n        Default: 42.0 42.0\n");
                std::process::exit(0);

            },

            s => { parse!(s, error) },

        }

    }

    // Compute coordinates of the frame of the Complex plane, if none have been provided
    if (output.1).0.r == 42.0 && (output.1).0.i == 42.0 && (output.1).1.r == 42.0 && (output.1).1.i == 42.0 {

        let step_size = 1f64 / zoom;
        let frame_width  = (output.0).0 as f64 * step_size;
        let frame_height = (output.0).1 as f64 * step_size;

        let delta = Complex::new(frame_width / 2f64, frame_height / 2f64);
        let c1 = center.sub(&delta);
        let c2 = center.add(&delta);

        (output.1).0 = c1;
        (output.1).1 = c2;

    }

    output

}


/// generates a partially random filename
///
/// `gen_filename()` generates a filename along the pattern `birb_XXXX.birb` where `XXXX` are four
/// randomly selected uppercase letters from the ASCII set.
#[inline]
fn gen_filename() -> String {

    use rand::distributions::Distribution;

    let mut r = rand::thread_rng();
    let dist  = rand::distributions::Uniform::from(65..90u8);

    let a = dist.sample(&mut r) as char;
    let b = dist.sample(&mut r) as char;
    let c = dist.sample(&mut r) as char;
    let d = dist.sample(&mut r) as char;

    format!("birb_{}{}{}{}.birb", a,b,c,d,)

}



/// Reads a `.birb` file to `Vec<u64>`.
/// Takes the `filename` to read from.
///
/// ### Possible Errors and Panics
///
/// If the `.birb` file doesn't contain full u64 numbers, it will be truncated to fit.
///
/// If the `.birb` file isn't formatted properly, that is, contains as many `u64` values as the
/// product of the first two `u64`s plus 2 (width * height + 2 (for the width and the height)),
/// this function exits the process with an error message.
///
/// If the `.birb` file couldn't be read, this function also exits the process, with a different
/// error message.
pub fn read_birb(filename: &str) -> Vec<u64> {

    /* Open and read the birb file */

    let mut f = error!(File::open(filename), "Couldn't open file. The specified birb-file doesn't exist or is inaccessible.", full);

    let mut birb_raw: Vec<u8> = Vec::new();

    error!(f.read_to_end(&mut birb_raw), "There was an error while reading the birb file.", full);


    /* Convert to u64 */

    // u64 is made of 8 bytes, so:
    let length = birb_raw.len() / 8;

    // Make birb_raw's capacity and length reflect the target capacity and length, so we don't
    // accidentally make the allocator forget about bits of birb_raw
    birb_raw.resize(length * 8, 0);
    birb_raw.shrink_to_fit();       // shrinks capacity as much as possible

    // Now the target capacity should be:
    let capacity = birb_raw.capacity() / 8;


    /* Do not confuse the allocator */

    // The vector is now as small as possible:
    assert!(capacity == length);

    // The vector really has capacity and length as capacity() and len() AND
    // the vector contains a clean multiple of 8 items (cause 8 bytes = 1 u64)!
    assert!(capacity * 8 == birb_raw.capacity() && birb_raw.capacity() % 8 == 0);
    assert!(length   * 8 == birb_raw.len()      && birb_raw.len()      % 8 == 0);



    // Make a u64 pointer to the birb
    let ptr8 = birb_raw.as_mut_ptr();
    let ptr  = ptr8 as *mut u64;

    let birb;

    unsafe {

        // Forget the old pointer, so we don't have two simultaneously...
        std::mem::forget(birb_raw);

        // Make a new vector
        birb = Vec::from_raw_parts(ptr, length, capacity);

        // Note: I know what I'm doing here is bad, but it's necessary.
        // Note: I'm sure there is a way to do this with transmute... I tried. But using transmute
        // would be absolutely terrible.

    }


    /* Validate birb format */

    // The first two numbers in a valid birb are its width and height, so their product is the
    // number of numbers stored in the rest of the birb. That product plus 2 for the first two
    // should equal the buffer's size exactly.
    // If there are less than two numbers, you obviously screwed up.
    if birb.len() < 2 || birb.len() as u64 != birb[0] * birb[1] + 2 {

        error!(Err("honk"), "The read birb file is malformed.");

    }

    birb

}

/// Writes a `.birb` file from an existing birb buffer.
/// Takes a `filename` to write to and a borrow of a `birb`, which is the data to write.
pub fn write_birb(filename: &str, birb: &Vec<u64>) {

    /* Open file to write to */

    let mut f = error!(File::create(filename), "Couldn't open birb file to write.", full);


    /* Convert from u64 to u8 */

    // Turns out, this can't actually be done using from_raw_parts() cause when you create
    // a Vec like that, the data might get changed or corrupted. That happened here....


    let mut birb_raw:Vec<u8> = Vec::with_capacity(birb.len()*8);

    for n in birb.iter() {

        // Get a u8 pointer to the data
        let ptr64 = n as *const u64;
        let ptr8  = ptr64 as *const u8;

        // a u64 is made from 8 u8 values
        for offset in 0..8 {

            unsafe {
                let ptr = ptr8.clone().offset(offset);
                birb_raw.push(*ptr);
            }

        }

    }


    /* Write */

    error!(f.write_all(&birb_raw), "There was an error while writing the birb file", full);

}
