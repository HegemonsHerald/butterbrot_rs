mod lib;
use lib::io::*;

fn main() {

    /* Try and read the birb file */

    let mut buffer: Vec<u64>;

    buffer = read_birb("foo.birb");

    /* how to make example data:
    let mut buffer:Vec<u64> = Vec::new();

    // width and height
    buffer.push(2);
    buffer.push(2);

    buffer.push(42);
    buffer.push(420);
    buffer.push(4200);
    buffer.push(42000);
    write_birb("data.birb", &buffer);
    */

    /* Try and write the birb file */

    write_birb("birb.birb", &buffer);


    /* Finish properly */

    std::process::exit(0);

}
