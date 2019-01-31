/// Macro to help with parsing command line arguments.
///
/// ### What does this do?
///
/// This Macro has three rules:
///
/// 1. Complex Numbers: `parse!("-c1", args, complex)`
/// 2. Strings: `parse!("--filename", args, string)`
/// 3. Primitives: `parse!("--width", args, u64)`
///
/// All three variants take a token `&str`, which should be the name of the command line flag, that
/// is currently being parsed. (For error output)
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
/// let mut curr;
/// loop {
///
///     match args.next() {
///         Some(s) => curr = s,
///         None    => break,
///     }
///
///     match curr {
///
///         "-w" || "--width"  => { output.0  = parse!("--width",    args, i32) },
///         "-h" || "--height" => { output.1  = parse!("--height",   args, i32) },
///         "--filename"       => { output.3  = parse!("--filename", args, string) },
///         "--complex"        => {
///             let (r,i) = parse!("--complex", args, complex);
///             output.2  = math::Complex::new(r, i)
///         },
///
///         s => { panic!("Invalid argument encountered: {}", s) },
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
                println!("\x1B[31;1mError:\x1B[0m The flag '{}' expects two arguments, a real part and an imaginairy part.", $token);
                panic!("");
            });

            // Get real part of the complex number
            let i = $source.next().unwrap_or_else(|| {
                println!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one more argument, the imaginairy part.", $token);
                panic!("");
            });

            // Parse the next token
            let rp = r.parse::<f64>().unwrap_or_else(|_| {
                println!("\x1B[31;1mError:\x1B[0m Couldn't parse the real part {real} as f64 in '{token} {real} {imag}'.", real = r, imag = i, token = $token);
                panic!("");
            });

            // Parse the next token
            let ip = i.parse::<f64>().unwrap_or_else(|_| {
                println!("\x1B[31;1mError:\x1B[0m Couldn't parse the imaginairy part {imag} as f64 in '{token} {real} {imag}'.", real = r, imag = i, token = $token);
                panic!("");
            });

            (r, i)
        }
    };

    // Parse Flag with stringy argument
    ($token:expr, $source:ident, string) => {
        {

            // Override panic! message
            std::panic::set_hook(Box::new(|_| {}));

            // Get the next token, which is a String
            $source.next().unwrap_or_else(|| {
                println!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one argument.", $token);
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
                println!("\x1B[31;1mError:\x1B[0m The flag '{}' expects one argument.", $token);
                panic!("");
            });

            // Parse the next token
            let i = t.parse::<$type>().unwrap_or_else(|_| {
                println!("\x1B[31;1mError:\x1B[0m Couldn't parse {arg} as {type} in '{token} {arg}'.", arg = t, token = $token, type = stringify!($type));
                panic!("");
            });

            i
        }
    }
}

fn main() {

    let v = vec!("33".to_string(), "2".to_string(), "ouija".to_string(), "22.0".to_string(), "2938.923874".to_string());
    let mut vi = v.into_iter();
    let mut output = (0, 0, "hhh".to_string(), (32, 98));

    let d = "honk".to_string();
    let dd = &d;
    let donk = &dd;
    println!("{}", donk);

    println!("output: {:?}", output);

    output.0 = parse!("honk", vi, i32);
    (output.3).0 = parse!("honk",  vi, i32);
    output.2 = parse!("honk", vi, string);

    println!("output: {:?}", output);

    let (a,b) = parse!("c1", vi, complex);
    println!("Complex {{ r: {}, i: {} }}", a, b);

    // So, this macro I created now can do this:
    //
    // It can produce generic primitives form args!
    // It can produce a pair of f64 for making a complex number!
    // It can return a String, when a String is needed.

    /*
     *
     * So now the args parser will do sth like this:
     *
     * let mut args = arg_v.into_iter();
     *
     * match args.next() {
     *
     *      "-w"  || "--width"      => { (output.0).0 = parse!("--width",  args, u64) },
     *      "-h"  || "--height"     => { (output.0).1 = parse!("--height", args, u64) },
     *      "-it" || "--iterations" => { output.1 = parse!("--iterations", args, i32) },
     *      "-s"  || "--samples"    => { output.1 = parse!("--samples",    args, i32) },
     *
     *      // ...
     *
     * }
     *
     */

}
