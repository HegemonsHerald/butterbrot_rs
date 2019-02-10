mod lib;

use lib::io::read_birb;

fn main() {

    let birb = read_birb("data.birb");

    let w = birb[0];
    let h = birb[1];

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


}
