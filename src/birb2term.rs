mod lib;

use lib::io::read_birb;

fn main() {

    let birb = read_birb("data.birb");

    let w = birb[0];
    let h = birb[1];



	/* The good parts.
	 *
	 * Render 'Honk4.birb' using the first section's for loops.
	 * This one's the first time, I made the Buddhabrot apparent.
	 * This one's gotten a weird resolution, so does Honk7.
	 *
	 * Render 'Honk7.birb' using the second section's for loops.
	 * This one's gotten about 20 times the resolution of honk4.
	 *
	 * Render 'Honk8.birb' using the third section's for loops.
	 * This one's the first with a round resolution: 1000x1000
	 *
	 */




    // good render settings:
    // --samples 10_000_000 --iterations 200 and whatever warmup it was, that I set as default right now (I think 400, or maybe 10)


/*****************************************************/

	// works well with data set 'honk4' -- the first truly successfull run!
    for i in 0..h {

        for j in 2..w {

		let c = birb[(i*w+j-2) as usize];
		if c > 50 && c < 150 {
			print!("   ∙    ");
		} else if c > 150 && c < 1000 {
			print!("   ○    ");
		} else if c > 500 && c < 1000 {
			print!("   ◆    ");
		} else if c > 1000 && c < 5000 {
			print!("░░░░░░░░");
		} else if c > 5000 && c < 10000 {
			print!("▒▒▒▒▒▒▒▒");
			// print!("{:^10}", c);
		} else if c > 10000 && c < 20000{
			print!("▓▓▓▓▓▓▓▓");
		} else if c > 20000 {
			print!("████████");
		}
            // print!("{:^10}", (birb[(i*w+j) as usize] as f64).sqrt() as u64);
            // print!("{:^10}", (birb[(i*w+j) as usize]) as u64);

        }

        print!("\n");

    }

/*****************************************************/

    for i in 0..h {

        for j in 2..w {

		let c = birb[(i*w+j-2) as usize];
		if c > 50 && c < 150 {
			print!(" ∙ ");
		} else if c > 150 && c < 1000 {
			print!(" ○ ");
		} else if c > 500 && c < 1000 {
			print!(" ◆ ");
		} else if c > 1000 && c < 5000 {
			print!("░░░");
		} else if c > 5000 && c < 10000 {
			print!("▒▒▒");
		} else if c > 10000 && c < 15000{
			print!("▓▓▓");
		} else if c > 15000 {
			print!("███");
		} else {
			print!("   ");
		}

        }

        print!("\n");

    }

/*****************************************************/

    for i in 0..h {

        for j in 2..w {

		let c = birb[(i*w+j-2) as usize];
		if c > 50 && c < 150 {
			print!("∙ ");
		} else if c > 150 && c < 1000 {
			print!("○ ");
		} else if c > 500 && c < 1000 {
			print!("■ ");
		} else if c > 1000 && c < 5000 {
			print!("░░");
		} else if c > 5000 && c < 10000 {
			print!("▒▒");
		} else if c > 10000 && c < 15000{
			print!("▓▓");
		} else if c > 15000 {
			print!("██");
		} else {
			print!("  ");
		}

        }

        print!("\n");

    }

/*****************************************************/

    for i in 0..h {

        for j in 2..w {

		let c = birb[(i*w+j-2) as usize];
		if c > 50 && c < 150 {
			print!("∙ ");
		} else if c > 150 && c < 1000 {
			print!("○ ");
		} else if c > 1000 && c < 20000 {
			print!("■ ");
		} else if c > 20000 && c < 80000 {
			print!("░░");
		} else if c > 80000 && c < 150000 {
			print!("▒▒");
		} else if c > 150000 && c < 200000{
			print!("▓▓");
		} else if c > 200000 {
			print!("██");
		} else {
			print!("  ");
		}

        }

        print!("\n");

    }


}
