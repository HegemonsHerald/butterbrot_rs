
/* The Complex Number Type */

/// represents a complex number, use field `r` to access real part and field `i` for imaginairy
/// part.
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub r: f64,
    pub i: f64,
}

impl Complex {
    /// well, what could this function possibly do?
    pub fn new(r:f64, i:f64) -> Complex {
        Complex { r, i }
    }

    #[inline]
    fn multiply(&self, c:&Complex) -> Complex {

        let r = self.r * c.r - self.i * c.i;
        let i = self.r * c.i + self.i * c.r;

        Complex::new(r, i)

    }

    #[inline]
    fn squared(&self) -> Complex {
        self.multiply(self)
    }

    #[inline]
    fn add(&self, c:&Complex) -> Complex {

        let r = self.r + c.r;
        let i = self.i + c.i;

        Complex::new(r, i)

    }

}


/* The Mandelbrot Orbit Type */

/// iterator, that yields the sequence of `Complex` numbers, produced by repeatedly apply-ing the
/// mandelbrot equation to the iterator's internal "last complex number", starting from the complex
/// number passed to `Orbit::new()`
#[derive(Clone, Copy)]
pub struct Orbit {
    c: Complex,
    z: Complex,
    n: i32,
}

impl Orbit {

    /// creates a new `Orbit` starting at `c`, which yields `n` complex numbers
    pub fn new(c:Complex, n:i32) -> Orbit {
        let z = Complex::new(0f64, 0f64);
        Orbit { c, z, n }
    }

    /// the actual mandelbrot function
    #[inline]
    fn mandelbrot(c:&Complex, z:&Complex) -> Complex {

        let zz = z.squared();

        c.add(&zz)

    }

}

impl Iterator for Orbit {

    type Item = Complex;

    /// returns the next `Complex` number of this `Orbit`'s sequence, unless the `Orbit` has been
    /// consumed entirely
    fn next(&mut self) -> Option<Self::Item> {

        if self.n > 0 {

            self.n -= 1;

            self.z = Orbit::mandelbrot(&self.c, &self.z);

            return Some(self.z);
        }

        None

    }

}


/* The Metropolis-Hastings Orbit Collection Type */

/// Metropolis-Hastings Orbits Iterator.
/// This iterator yields Buddahbrot `Orbit`s for a specified number of samples, where a sample is a
/// complex number. The samples are chosen using an adapted Metropolis-Hastings method, hence the
/// name. The first sample will be chosen randomly. The yielded `Orbit`s will for the most part be
/// relevant to the Buddahbrot fractal.
///
/// Note that due to implementation and the functionality of Metropolis-Hastings running through
/// MHOrbits requires computing possibly quite a bit of `Orbit`s, that are ultimately discarded...
pub struct MHOrbits {

    sample_count: i32,
    sample: Complex,    // the previous sample
    length: i32,        // length of the previous sample

    iterations: i32,    // how long to make each singular orbit at max

    // the rectangle of the complex plane, we wish to explore
    lower_bound: Complex,
    upper_bound: Complex,

}


impl MHOrbits {

    /// Creates a new MHOrbits Iterator
    ///
    /// ### Arguments
    /// `sample_count` is the number of (*computed*) orbits this iterator will yield
    ///
    /// `warmup` is the number of samples to compute and discard, to "warm-up"
    /// Metropolis-Hastings
    ///
    /// `iterations` is the number of iterations, each orbit will test for
    ///
    /// `corner_1` is the complex number of one of the corners of the rectangular segment of
    /// Buddahbrot, we'd like to explore
    ///
    /// `corner_2 is the complex nuymber of the corner diagonally opposite of `corner_1`
    ///
    /// **Note:** The Orbits this iterator yields, will be *computed*, that is, they aren't actually
    /// `Orbit`-type Iterators, but the results of such, collected into `Vec<Complex>`-type
    /// Vectors!
    pub fn new(sample_count:i32, warmup:i32, iterations:i32, lower_bound: Complex, upper_bound: Complex) -> MHOrbits {

        /* Create a new MHOrbits */

        // Create orbit, filter it for 'interesting' numbers, and figure out its length (the actual numbers don't matter, cause warm-up)
        let sample: Complex = MHOrbits::rnd_sample();
        let length = Orbit::new(sample, iterations)
            .filter(|c| MHOrbits::in_range(&c, &lower_bound, &upper_bound))
            .collect::<Vec<Complex>>()              // TURBOOOO FIIIIIISH, YAY =)
            .len() as i32;

        let mut mho = MHOrbits {

            sample_count: sample_count + warmup,

            sample,
            length,

            iterations,
            lower_bound,
            upper_bound,

        };


        /* Warm Up the MHOrbits Iterator */

        // Metropolis-Hastings likes to be 'warmed up'. That means it's good practice to discard
        // the first couple of thousands of samples, because it's possible, that you randomly
        // started sampling from a low-interest position and need to make your way to a
        // high-interest position first...

        // nth() **consumes** the first (n-1) elements and yields the nth, which we can promptly ignore!
        mho.nth(warmup as usize);


        /* Yield the Warmed-Up Iterator */

        mho

    }


    /// Whether a complex number is in the range, we want to explore, or not
    #[inline]
    fn in_range(c:&Complex, lower_bound:&Complex, upper_bound:&Complex) -> bool {

        if lower_bound.r <= c.r && c.r < upper_bound.r
        && lower_bound.i <= c.i && c.i < upper_bound.i {
            return true
        }

        false

    }

    /// Whether or not to discard the current Metropolis-Hastings sample
    #[inline]
    fn discard(sample1:&Complex, len1:i32, sample2:&Complex, len2:i32) -> bool {
        // TODO impl logic for choosing to transition or not
        false
    }

    /// Chooses a random complex number not in the Mandelbrot set, but somewhere in its vicinity
    fn rnd_sample() -> Complex {
        // TODO choose an actually random number
        Complex::new(-0.25, -0.25)
    }

    /// Creates a random complex number not in the Mandelbrot set, by randomly offseting the
    /// complex number `c`
    fn sample_from(c:&Complex) -> Complex {
        // TODO choose a good random number based on the previous random number
        Complex::new(c.r, c.i)
    }

    /// Tells you how many samples are still left from this `MHOrbits`
    pub fn remaining(&self) -> i32 {
        self.sample_count
    }

    /// Tells you whether this `MHOrbits` has finished its computation, without you needing to
    /// create a peekable Iterator or -- god fobid -- pattern match against `next()`
    pub fn finished(&self) -> bool {
        if self.sample_count > 0 {
            return false;
        } else {
            return true;
        }
    }

}

impl Iterator for MHOrbits {

    type Item = Vec<Complex>;

    /// returns a `Vec<Complex>`, which contains the numbers of the next successfull `Orbit`
    fn next(&mut self) -> Option<Self::Item> {

        if self.sample_count > 0 {

            self.sample_count -= 1;

            loop {

                /* Compute a new sample and orbit */

                let s = MHOrbits::sample_from(&self.sample);
                let o = Orbit::new(s, self.iterations)
                    .filter(|c| MHOrbits::in_range(c, &self.lower_bound, &self.upper_bound))
                    .collect::<Vec<Complex>>();

                let l = o.len() as i32;


                /* Maybe discard it? */

                if MHOrbits::discard(&s, l, &self.sample, self.length) {
                    continue;
                } else {
                    self.sample = s;
                    self.length = l;

                    return Some(o);
                }

            }

        }

        None

    }

}
