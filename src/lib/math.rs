extern crate rand;
use rand::Rng;
use rand::prelude::ThreadRng;

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
    fn abs(&self) -> f64 {

        (self.r.powi(2) + self.i.powi(2)).sqrt()

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
    step_size: [f64; 2],

    rng: ThreadRng,

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
    pub fn new(sample_count:i32, warmup:i32, iterations:i32, step_size:[f64;2], lower_bound: Complex, upper_bound: Complex) -> MHOrbits {

        /* Create a new MHOrbits */

        // Create a random number generator for choosing samples
        let mut rng = rand::thread_rng();

        // Create orbit, filter it for 'interesting' numbers, and figure out its length (the actual numbers don't matter, cause warm-up)
        let sample: Complex = MHOrbits::rnd_sample(&mut rng);
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
            step_size,

            rng,

        };


        /* Warm Up the MHOrbits Iterator */

        // Metropolis-Hastings likes to be 'warmed up'. That means it's good practice to discard
        // the first couple of thousands of samples, because it's possible, that you randomly
        // started sampling from a low-interest position and need to make your way to a
        // high-interest position first...

        // nth() **consumes** the first (n-1) elements and yields the nth, which we can promptly ignore!
        mho.nth(warmup as usize);

        println!("Got through Warmup");


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
    /// This is a front-end to the TransitionProbability function from the reference
    #[inline]
    fn discard(rng:&mut ThreadRng, iterations:i32, len1:i32, len2:i32) -> bool {

        let t_prob_1  = MHOrbits::transition_probability(iterations, len1, len2);
        let t_prob_2  = MHOrbits::transition_probability(iterations, len2, len1);
        let contrib_1 = MHOrbits::contribution(iterations, len1);
        let contrib_2 = MHOrbits::contribution(iterations, len2);

        let a = (contrib_1 * t_prob_1).log10();
        let b = (contrib_2 * t_prob_2).log10();
        let c = (a-b).exp();
        let alpha = 1f64.min(c);

        let r = rng.gen_range(0f64, 1f64);

        alpha < r

    }

    /// How strongly a particular `Orbit` contributes, as a percentage from its maximum length
    #[inline]
    fn contribution(iterations:i32, len:i32) -> f64 {

        len as f64 / iterations as f64

    }

    #[inline]
    fn transition_probability(iterations:i32, len1:i32, len2:i32) -> f64 {

        (1f64 - (iterations - len1) as f64 / iterations as f64) /
        (1f64 - (iterations - len2) as f64 / iterations as f64)

    }

    /// Chooses a random complex number not in the Mandelbrot set, but somewhere in its vicinity
    #[inline]
    fn rnd_sample(rng:&mut ThreadRng) -> Complex {
        loop {

            let real = rng.gen_range(-2f64, 2f64);
            let imag = rng.gen_range(-2f64, 2f64);
            let c = Complex::new(real, imag);

            // Figure out, if c is inside the mandelbrot set:
            let o = Orbit::new(c, 400);
            if o.last().unwrap().abs() < 2f64 { continue }
            else { return c }

        }

    }

    /// Creates a random complex number not in the Mandelbrot set, by randomly offseting the
    /// complex number `c`
    #[inline]
    fn sample_from(rng:&mut ThreadRng, step_size:[f64;2], c:&Complex) -> Complex {

        // Randomness every now and so often!
        // NOTE gen_range(low,high) with inclusive low and exclusive high
        if rng.gen_range(0,6) > 4 {

            return MHOrbits::rnd_sample(rng);

        }

        // Compute offset factors for real and imaginairy parts

        // Factors that make the offset depend on the step_size
        let r1 = 1f64 / (1f64 / step_size[0]) * 0.0001;
        let r2 = 1f64 / (1f64 / step_size[0]) * 0.01;
        let i1 = 1f64 / (1f64 / step_size[1]) * 0.0001;
        let i2 = 1f64 / (1f64 / step_size[1]) * 0.01;

        // Actual Factors for weighting the random offset
        let r_real = r1 * -(r2/r1).log10() * rng.gen_range(0f64,1f64);
        let r_imag = i1 * -(i2/i1).log10() * rng.gen_range(0f64,1f64);

        // Get a nice random factor
        let phi = rng.gen_range(0f64,1f64) * std::f64::consts::PI * 2f64;


        // Create the new Complex number
        let real = c.r + r_real * phi.cos();
        let imag = c.i + r_imag * phi.sin();

        Complex::new(real, imag)

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

                let s = MHOrbits::sample_from(&mut self.rng, self.step_size, &self.sample);

                // TODO Check whether the very last point of the orbit is going off to infinity

                let mut o = Orbit::new(s, self.iterations)
                    .enumerate()
                    .filter(|(i,c)| {
                        if i+1 == self.iterations as usize { return true }
//                      .filter(|c| {
                        MHOrbits::in_range(c, &self.lower_bound, &self.upper_bound)
                    })
                    .map(|(_,c)| c)
                    .collect::<Vec<Complex>>();

                // If the sample still turns out to be bad...
                let last = o[o.len()-1];
                if last.abs() < 2f64 {
                    continue;
                } else if !MHOrbits::in_range(&last, &self.lower_bound, &self.upper_bound) {
                    o.pop();
                }

                let l = o.len() as i32;


                /* Maybe discard it? */

                if MHOrbits::discard(&mut self.rng, self.iterations, l, self.length) {
                    // println!("discarded");
                    continue;
                } else {
                    // println!("SUCCESS");
                    self.sample = s;
                    self.length = l;

                    return Some(o);
                }

            }

        }

        None

    }

}
