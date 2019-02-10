mod lib;

use lib::io::read_birb;

fn main() {

    let birb = read_birb("data.birb");

    let w = birb[0];
    let h = birb[1];

    for i in 0..h {

        for j in 0..w {

            // print!("{:^10}", (birb[(i*w+j) as usize] as f64).sqrt() as u64);
            print!("{:^10}", (birb[(i*w+j) as usize]) as u64);

        }

        print!("\n");

    }


}
