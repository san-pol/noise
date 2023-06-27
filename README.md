# noise
A simple Rust implementation of fractal noise.

Creates a 2D array of float 32s to encode color information. The noise is layered white noise, with one layer containing one octave of noise.
A shift in an octave causes a doubling of noise frequency.

The resulting fractal noise is visually reminiscent of a cloud or fog.

The code uses a custom hash function to generate random numbers. The origin of the function is unknown to me.
A simple test function checks to see if there are any obvious hash collisions.

The size of generated fractal noise images is fixed, being a square with side length 2^(n_octaves - 1) pixels.
