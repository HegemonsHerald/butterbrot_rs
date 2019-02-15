extern crate rayon;
#[macro_use]
extern crate bmp;

mod lib;
use lib::io::*;
use rayon::prelude::*;
use bmp::Pixel;

use std::time::Instant;


fn main() {

    /* Figure out, what to read and write */

    let args:Vec<String> = std::env::args().collect();

    let mut filename = gen_filename("bmp");
    let mut src_name = "data.birb".to_string();
    let mut invert   = false;

    match args.len() {
        0 => {},
        1 => {},
        2 =>   src_name = args[1].clone(),
        3 => { src_name = args[1].clone(); filename = args[2].clone() },
        _ => { src_name = args[1].clone(); filename = args[2].clone(); invert = true },
    }



    /* Get the data */

    let mut birb = read_birb(&src_name);

    let width  = birb[0];
    let height = birb[1];


    // Find largest value
    let max;
    match &mut birb[2..].par_iter().max() {
        Some(&m) => max = m,
        None     => max = 0
    }


    // What to divide by to map to 256
    let mapper = max / 255;



    /* Make an image out of it */

    let mut img = bmp::Image::new(width as u32, height as u32);

    &birb[2..]

        // Map to 256 range
        .par_iter()
        .enumerate()
        .map(|(i,n)| (i, n / mapper))

        // Add to image as pixel
        .collect::<Vec<_>>().into_iter()
        .for_each(|(i,n)| {

            let x = (i % width as usize) as u32;
            let y = (i / width as usize) as u32;

            let n = if invert { 255 - n } else { n };

            img.set_pixel(x, y, px!(n, n, n));

        });

    // Write it to file
    let _ = img.save(filename);


}
