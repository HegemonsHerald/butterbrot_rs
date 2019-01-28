pub mod io;
pub mod math;

// TODO proper re-exports
// TODO make private, what can be private


#[cfg(test)]
mod tests {

    use super::io::*;
    use super::math::*;

    #[test]
    fn read_write() {

        /* Try and read the birb file */
        let buffer: Vec<u64> = read_birb("foo.birb");

        /* Try and write the birb file */
        write_birb("birb.birb", &buffer);

    }

    #[test]
    fn make_example_data() {

        let mut buffer:Vec<u64> = Vec::new();

        // width and height
        buffer.push(2);
        buffer.push(2);

        buffer.push(42);
        buffer.push(420);
        buffer.push(4200);
        buffer.push(42000);

        // this thing won't write somehow...
        // write_birb("data.birb", &buffer);
    }

    #[test]
    fn orbit_struct() {

        let orbit = Orbit::new(Complex{r:33.0,i:32.0}, 10);

        for i in orbit {
            println!("Complex number:\t{{ real: {}, imag: {} }}", i.r, i.i);
        }

    }

    #[test]
    fn mh_orbits_struct() {

        let mut mh_orbit = MHOrbits::new(5, 30, 4, Complex::new(-1f64, -7.0), Complex::new(10.0, 10.0));

        for i in mh_orbit {
            let mut vv: Vec<Complex>;
            println!(":=========================================:");

            for j in i {
                println!("Complex number:\t{{ real: {}, jmag: {} }}", j.r, j.i);
            }

        }

    }


}
