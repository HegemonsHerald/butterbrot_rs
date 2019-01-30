macro_rules! error {

    ( $( $result:expr, $msg:expr ),* ) => {
        $($result)*.unwrap_or_else(|_e| {
            print!("HONK: ");
            println!("{}", $($msg)*);
            std::process::exit(1);
        })
    }

}


fn main() {

    let m = Err("honk");

    error!(m, "GRRRR");

}


