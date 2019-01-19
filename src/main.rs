mod lib;

use lib::*;



fn main() {

    /* Try and read the birb file */

    let mut buffer: Vec<u64> = Vec::new();

    match io::read_birb("foo.birb") {
        Ok(v)  => buffer = v,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::InvalidInput => println!("\x1B[31;1mError:\x1B[0m Couldn't read, the specified birb-file doesn't exist or is inaccessible."),
                std::io::ErrorKind::InvalidData  => println!("\x1B[31;1mError:\x1B[0m Couldn't read, the specified birb file is malformed."),
                _ => println!("\x1B[31;1mError:\x1B[0m {:?}", e)
            }
            std::process::exit(1)
        }
    }


    /* Try and write the birb file */

    /* how to make example data:
    let mut buffer:Vec<u64> = Vec::new();

    // width and height
    buffer.push(2);
    buffer.push(2);

    buffer.push(42);
    buffer.push(420);
    buffer.push(4200);
    buffer.push(42000);
    lib::io::write_birb("data.birb", &buffer);
    */

    match lib::io::write_birb("birb.birb", &buffer) {
        Ok(()) => {},
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists    => println!("\x1B[31;1mError:\x1B[0m Couldn't write, the specified output birb-file already exists."),
                std::io::ErrorKind::PermissionDenied => println!("\x1B[31;1mError:\x1B[0m Couldn't write, permission denied."),
                _ => println!("\x1B[31;1mError:\x1B[0m {:?}", e)
            }
            std::process::exit(1)
        }
    }

    std::process::exit(0);

}
