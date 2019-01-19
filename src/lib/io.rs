use std::io::*;
use std::fs::File;


/// Reads a `.birb` file to `Vec<u64>`.
/// Takes the `filename` to read from.
///
/// If the `.birb` file isn't formatted properly, that is, contains as many `u64` values as the
/// product of the first two `u64`s plus 2 (width * height + 2 (for the width and the height)),
/// this function returns an `std::io::ErrorKind::InvalidData`.
///
/// If the `.birb` file couldn't be read, this returns an `std::io::ErrorKind::InvalidInput`.
pub fn read_birb(filename: &str) -> Result<Vec<u64>> {

    /* Open and read the birb file */

    let mut f;

    match File::open(filename) {
        Err(_e) => return Err(Error::from(ErrorKind::InvalidInput)),
        Ok(v)  => f = v,
    }

    let mut birb_raw: Vec<u8> = Vec::new();

    f.read_to_end(&mut birb_raw)?;


    /* Convert to u64 */

    let mut birb: Vec<u64> = Vec::new();

    // For all groups of 8 bytes in the buffer...
    for i in 0..(birb_raw.len() / 8) {

        // ... turn them into one u64...
        let ptr8  = &birb_raw[i*8] as *const u8;
        let ptr64 = ptr8 as *const u64;

        // ... and add that u64 to the birb
        unsafe {
            birb.push(*ptr64);
        }

    }


    /* Validate birb format */

    if birb.len() as u64 != birb[0] * birb[1] + 2 {

        return Err(Error::from(ErrorKind::InvalidData));

    }

    Ok(birb)

}

pub fn write_birb(filename: &str, birb: &Vec<u64>) -> Result<()> {

    /* Open file to write to */

    let mut f = File::create(filename)?;


    /* Convert from u64 to u8 */

    let mut birb_raw: Vec<u8> = Vec::new();

    for i in 0..(birb.len()) {

        // Make pointer to the i-th u64
        let ptr64 = &birb[i] as *const u64;
        let ptr8  = ptr64 as *const u8;

        for offset in 0..8 {

            unsafe {
                let ptr = ptr8.clone().offset(offset);
                birb_raw.push(*ptr);
            }

        }

    }


    /* Write */

    f.write_all(&birb_raw)?;

    Ok(())

}

// pub fn filename(prefix: String) -> String {

// }

