use std::io::*;
use std::fs::File;
extern crate rand;

/// generates a partially random filename
///
/// `gen_filename()` generates a filename along the pattern `birb_XXXX.birb` where `XXXX` are four
/// randomly selected uppercase letters from the ASCII set.
///
/// Takes an extension as argument, that will be added as ".extension" to the end of the random
/// filename.
#[inline]
pub fn gen_filename(ext:&str) -> String {

    use rand::distributions::Distribution;

    let mut r = rand::thread_rng();
    let dist  = rand::distributions::Uniform::from(65..90u8);

    let a = dist.sample(&mut r) as char;
    let b = dist.sample(&mut r) as char;
    let c = dist.sample(&mut r) as char;
    let d = dist.sample(&mut r) as char;

    format!("birb_{}{}{}{}.{}", a,b,c,d,ext)

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
