# integral_lib
Simple liblary for solving integrals using numerical methods

## examples:
```
2*simpsons_three_eights(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
```
will give you aproximation of Pi

## features:
there is `simd` feature that requires AVX support and nightly