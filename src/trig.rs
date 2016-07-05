pub fn sin(x: f64) -> f64 {
    let atol: f64 = 1e-12;
    let mut diff: f64 = 1.0;
    let mut sin: f64 = 0.0;
    let mut i: u8 = 1;
    while abs(diff) > atol {
        diff = (sin_neg(i) as f64) * pow(x,i) / (fact(i) as f64);
        sin += diff;
        i += 2;
    }
    sin
}

fn abs(n: f64) -> f64 {
    if (n as i64) < 0 {
        n * -1.0
    } else {
        n
    }
}

fn fact(z: u8) -> u64 {
    if z == 0 {
        1
    } else {
        (z as u64) * fact(z - 1)
    }
}

fn pow(x: f64, n: u8) -> f64 {
    if n == 0 {
        1.0
    } else {
        x * pow(x, n - 1)
    }
}

fn sin_neg(i: u8) -> i8 {
    if i % 4 == 1 {
        1
    } else {
        -1
    }
}
