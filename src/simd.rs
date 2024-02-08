#[cfg(feature = "simd")]
use packed_simd::Simd;

use std::mem::swap;

pub fn trapezoidal(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    if a > b {
        swap(&mut a, &mut b);
    }

    let h = (b - a) / steps as f64;
    let hh = 2f64 * h;
    let hhh = 3f64 * h;

    let mut sum = Simd::<[f64; 4]>::new(f(a), f(b), 0.0, 0.0) / 2f64;

    let mut x = a + h;
    while x < b {
        sum += Simd::<[f64; 4]>::new(f(x), f(x + h), f(x + hh), f(x + hhh));
        x += 4f64 * h;
    }

    let flat_sum: f64 = sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3);

    return h * flat_sum;
}

pub fn simpsons_one_third(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    let stepsf64 = steps as f64;

    if a > b {
        swap(&mut a, &mut b);
    }

    let h = (b - a) / (stepsf64);
    let hh = 2f64 * h;
    let hhh = 3f64 * h;

    let mut base_sum = (f(a) + f(b)) / 2f64;
    let mut sum = Simd::<[f64; 4]>::new(0.0, 0.0, 0.0, 0.0);
    let mut x = a + h;
    while x < b {
        sum += Simd::<[f64; 4]>::new(f(x), f(x + h), f(x + hh), f(x + hhh));
        x += 4f64 * h;
    }
    base_sum +=
        ((sum.extract(0) + sum.extract(2)) * 4f64) + ((sum.extract(1) + sum.extract(3)) * 2f64);

    return (h * base_sum) / 3f64;
}

pub fn simpsons_three_eights(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    let stepsf64 = steps as f64;

    if a > b {
        swap(&mut a, &mut b);
    }

    let h = (b - a) / stepsf64;
    let hh = 2f64 * h;

    let mut base_sum = f(a) + f(b);
    let mut sum = Simd::<[f64; 4]>::new(0.0, 0.0, 0.0, 0.0);
    let mut x = a + h;
    while x < b {
        let mut empt = [0f64, 0f64, 0f64, 0f64];
        for i in 0..3 {
            let val = f(x + (i as f64*h));
            if val.is_nan() {
                break; 
            }  
            empt[i] = val;
        }
        sum += Simd::from(empt);
        x += 3f64 * h;
    }

    base_sum += ((sum.extract(0) + sum.extract(1)) * 3f64) + (sum.extract(2) * 2f64);

    return h * base_sum * 3f64 / 8f64;
}

#[cfg(test)]
mod tests_simd {

    use super::*;
    // tests are made for aproximating equation:
    // pi = 2 * integral (-1, 1) sqrt(1-x^2) dx

    #[test]
    fn test_trapezoidal() {
        println!(
            "simd trapezoidal: {}",
            trapezoidal(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }

    #[test]
    fn test_simpsons_one_third() {
        println!(
            "simd one third: {}",
            simpsons_one_third(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }

    #[test]
    fn test_simpsons_three_eights() {
        println!(
            "simd three eights: {}",
            simpsons_three_eights(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }
}
