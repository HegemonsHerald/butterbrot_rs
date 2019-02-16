mod lib;
use lib::io::*;
use std::time::Instant;

const ERR_MSG:&str = "Not enough arguments provided.\n\n\tUSAGE:\n\n\t\tbirb_combinator birb1 birb2\n\n\t\tbirb_combinator outname birb1 birb2 [...]\n\n\tIf 2 birb files are provided, a random filename will be used.\n\tIf 3 or more arguments are provided, the first argument must be the filename to write the combined birb to.";

fn main() {

    /* Parse Input */

    let args:Vec<String> = std::env::args().collect();

    let filename;
    let mut buffer; // birb to sum other birbs into
    let rest;       // slice over the filenames of the birbs to add... the rest of the arguments

    match args.len() {

        1 => { error!(Err(()), ERR_MSG); panic!("") },
        2 => { error!(Err(()), ERR_MSG); panic!("") },

        // Two source files provided
        3 => {
            filename = gen_filename("birb");
            buffer   = read_birb(&args[1]);
            rest     = &args[2..];
        },

        // An out filename and source files provided
        _ => {
            filename = args[1].clone();
            buffer   = read_birb(&args[2]);
            rest     = &args[3..];
        }

    }



    /* Combine all the files, provided they are all of same width and height */

    let width  = buffer[0];
    let height = buffer[1];

    for b in rest.iter() {

        let buffer2 = read_birb(b);

        // Are the buffers at least somewhat compatible?
        if buffer2.len() != (width * height + 2) as usize {
            println!("\x1B[31;1mError:\x1B[0m The birb file \"{}\" does not have width {} and height {}!\n\n\t{0} {{ width: {}, height: {} }}", b, width, height, buffer2[0], buffer2[1]);
            continue;
        }

        buffer.iter_mut()
            .enumerate()
            .for_each(|(i,n)| {

            // Don't overwrite width and height
            if i >= 2 {

                // Don't overflow while adding
                if let (m, false) = (*n).overflowing_add(buffer2[i]) {
                    *n = m;
                }

            }

        });
    }



    /* Write output */

    write_birb(&filename, &buffer);

}
