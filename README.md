# Butterbrot_rs

*Buddhabrot? Pah, Butterbrot!*

Butterbrot_rs is a program, that computes and renders the (Buddhabrot)[https://en.wikipedia.org/wiki/Buddhabrot] set.  
The Buddhabrot set isn't really a set at all, it is a fancy way of viewing the Mandelbrot set. For an explanation see below.

I wrote this thing entirely in Rust, trying to use the features of the language to my advantage.
Basically, this was a practice project with a focus on a couple of major things:

- write *good* code, that is code, that doesn't need much -- if any -- commentary
- write the maths part as declaratively as possible (I was learning Haskell, while making this)
- actually finish this thing ;)

### So, what in this crate, huh?

The Butterbrot_rs crate exports three binairies: `butterbrot`, `birb2bmp` and `birb_combinator`.
(Technically there also is `birb2term`, which is a very limited renderer I used to initially verify, that `butterbrot` actually worked.)

#### Butterbrot
`butterbrot` is the main program. It computes a buffer of Buddhabrot values and writes it into a `birb` file.

Basic usage:

```
> butterbrot -o <filename> -w <width> -h <height> -s <samples> -i <iterations> [--timeout <timeout in seconds>]
```
Note, that all of the options are optional; if none are provided, default values will be used.

For more, read the help text you get from calling:

```
> butterbrot h
```

***Aside:*** `birb` is short for "butterbrot integer raw buffer", which
describes quite well, what it is. The name is just my little joke.  A `birb`
file is literally just a list of all the values, that came out of the
computation, with the caveat, that the first two numbers are the width and the
height, using which the rest of the buffer can be indexed. All numbers in the
buffer must be `u64`.

#### Birb to Bitmap
`birb2bmp` converts a `birb` file to a bitmap image. Usage as follows:

```
> birb2term birb_file.birb
> birb2term birb_file.birb bitmap_file.bmp
```

The first example will convert `birb_file.birb` to a bitmap and store the resulting image in a file, that has a (partially) random name.  
The second example will store the resulting image in a file called `bitmap_file.bmp`.

#### Birb Combinator
`birb_combinator` allows you to sum up the values of multiple `birb` files. Note, that the `birb` files **must** have the same width and height,
otherwise the program will skip the `birb` file, that doesn't conform.  
Integer overflow is handled by replacing overflowing values with `std::u64::MAX`.

This program is designed for the following use scenario:

Say you want to compute a render with **a f\*ck-ton of samples**. That is going to take a while. But: The longer any operation runs, the
more likely it becomes to fail, in which case you'd loose all your data. And maybe you'd like to use your computer at some point in the
not too distant future....  
Well, you can just compute multiple `birb`s with the exact same settings and use `birb_combinator` to merge them all into one dataset.
That way you can e.g. have the computer run the computation over multiple separate nights and afterwards you get the data, as if you
ran it all continuously.

### Examples

### How this actually works

The Buddhabrot is what you get, when you do the following:

1. Take points, that **aren't** in the Mandelbrot set.
2. Compute an "Orbit": An Orbit is the sequence of points on the complex plane, that you get from iteratively applying the Mandelbrot
   equation `z' = z*z + c`.  
   Basically, you choose a random complex number `c`, which isn't in the mandelbrot set. Then you plug in `z=0i0` and `c` into the
   Mandelbrot equation and you get out a new number `z'`. You write that number down. Then you plug in `z'` where you put `z` before
   and you do the same over and over again, for a specified number of iterations.  
   With this, an Orbit is the sequence of points you wrote down, starting from `c`.
3. Compute a whole bunch of Orbits.
4. Make a probability distribution of how likely it is, that any given Orbit passes through any given point.
   Basically, take a rectangle on the Complex Plane and count how often any of your Orbits passes through the points in the rectangle.
   The probability distribution is what you get, from counting specifically how often each point in the rectangle was hit by a point
   in the Orbit -- or how likely it is, that a point from the rectangle is hit by a point from a randomly chosen Orbit.
5. Make a picture out of that.

Now we call the number `c` a sample (or maybe more acurately the entire Orbit associated with `c`) and the flags of `butterbrot` should
basically make a lot of sense.

##### Metropolis Hastings

Turns out, probability distributions are a bit of a pain to compute. If you *just* choose random samples, you're going to wait a long
while, before you get a clear image, so instead lets be clever about this and copy (this dude (Alexander Boswell))[http://www.steckles.com/buddha/], who implemented a take on what's
known as the "Metropolis-Hastings" Algorithm.

(I used Mr. Boswell's C++ based implementation as a reference on how to do it, and in order to understand Metropolis-Hastings. However,
my Rust program did end up quite a bit different, using Iterators instead of for loops, which, as it turns out, are truly amazing
little things!)

Basically, you start with a random sample `c` and do the whole Orbit thing and whatnot. Since you know, how large your rectangle on the
complex plane is, you can then easily figure out, how many points of the Orbit are actually in the rectangle-of-interest. Those are
good points, you like them. Based on that you can make a statement on how good the overall Orbit was: the more points in, the better.  
Now you take another sample, but not entirely random: Take a sample, that's very close to the previous sample. Look, whether this new
sample is better than the previous. If it is better (or at least within a margin equal or worse) you keep it and choose the next sample
based on this only slightly moved sample. If it is worse, you discard of the new sample and try again from the old sample, until you
find a good sample. If you randomly mix in completely randomly chosen other samples, you get pretty good results.

Phew.

The gist of the idea is this: Once we figured out where to find good samples, we can just stay there, cause good samples are probably
close to each other. But to keep it interesting, every couple of samples we mix in a totally random sample. Since we now know, what a
good sample is, we can make an educated decision on whether to go into that new direction or stay with our well known good samples.

Because we first need to figure out, what a good sample is, it's recommendable to discard the first couple of thousands samples right
off the bat: You might've started off with a really bad sample, that you use to judge, what a good sample is. That's no good. And you don't
know. Just ignore the first couple samples and let Metropolis-Hastings *warm up*.

To learn more on Metropolis-Hastings and the Buddhabrot set in general, I can only point to:
- (wikipedia)[http://en.wikipedia.org/wiki/Buddhabrot]
- (Alexander Boswell)[http://www.steckles.com/buddha/]
- (Benedikt Bitterli)[https://benedikt-bitterli.me/buddhabrot/]


##### The Iterators

Now, if you've ever used Rust, you probably know this: Iterators are amazing! And since Metropolis-Hastings and the Buddhabrot Algorithm
are both very iterative processes of course I used Iterators to make them happen. And you can use Iterators in such a beautiful and
concise manner, it's bloody amazing.

NO triple-nested for-loops, NO unnecessary code duplication, NO nonsense and types, that speak for themselves. I love it!
