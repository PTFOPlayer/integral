use std::mem::swap;

fn partation(a: f64, b: f64, steps: usize) -> Vec<f64> {
    let mut v = vec![];
    let step_by: f64 = (b - a) / steps as f64;
    let mut x = a + step_by;
    while x < b {
        v.push(x);
        x += step_by;
    }

    return v;
}

pub fn trapezoidal(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    if a > b {
        swap(&mut a, &mut b);
    }

    let v = partation(a, b, steps);

    let h = (b - a) / steps as f64;

    let mut sum = (f(a) + f(b)) / 2f64;
    for e in v {
        sum += f(e);
    }

    return h * sum;
}

pub fn simpsons_one_third(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    let stepsf64 = steps as f64;

    if a > b {
        swap(&mut a, &mut b);
    }

    let v = partation(a, b, steps);

    let h = (b - a) / (stepsf64 * 3f64);

    let mut sum = (f(a) + f(b)) / 2f64;
    let mut s_2 = 0f64;
    let mut s_4 = 0f64;
    for i in 0..v.len() {
        if (i + 1) % 2 == 1 {
            s_4 += f(v[i]);
        } else {
            s_2 += f(v[i]);
        }
    }
    sum += (s_2 * 2f64) + (s_4 * 4f64);

    return h * sum;
}

pub fn simpsons_three_eights(mut a: f64, mut b: f64, steps: usize, f: fn(f64) -> f64) -> f64 {
    let stepsf64 = steps as f64;

    if a > b {
        swap(&mut a, &mut b);
    }

    let v = partation(a, b, steps);

    let mut h = (b - a) / stepsf64;
    h *= 3f64 / 8f64;

    let mut sum = (f(a) + f(b)) / 2f64;
    let mut s_2 = 0f64;
    let mut s_3 = 0f64;
    for i in 0..v.len() {
        if (i + 1) % 3 == 0 {
            s_2 += f(v[i]);
        } else {
            s_3 += f(v[i]);
        }
    }
    sum += (s_2 * 2f64) + (s_3 * 3f64);

    return h * sum;
}

#[cfg(test)]
mod tests {

    use super::*;
    // tests are made for aproximating equation:
    // pi = 2 * integral (-1, 1) sqrt(1-x^2) dx

    #[test]
    fn test_trapezoidal() {
        println!(
            "trapezoidal: {}",
            trapezoidal(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }

    #[test]
    fn test_simpsons_one_third() {
        println!(
            "one third: {}",
            simpsons_one_third(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }

    #[test]
    fn test_simpsons_three_eights() {
        println!(
            "three eights: {}",
            simpsons_three_eights(-1f64, 1f64, 600, |x| (1f64 - (x * x)).powf(0.5))
        );
    }
}
