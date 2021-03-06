mod trig {
    pub fn sin(x: f64) -> f64 {
        sinos(x, 's')
    }

    pub fn cos(x: f64) -> f64 {
        sinos(x, 'c')
    }

    pub fn tan(x: f64) -> f64 {
        sin(x)/cos(x)
    }

    pub fn sec(x: f64) -> f64 {
        1/cos(x)
    }

    pub fn csc(x: f64) -> f64 {
        1/sin(x)
    }

    pub fn cot(x: f64) -> f64 {
        cos(x)/sin(x)
    }

    fn sinos(x: f64, f: char) -> f64 {
        let atol: f64 = 1e-12;
        let mut diff: f64 = 1.0;
        let mut var: f64 = 0.0;
        let mut i: u8 = 0;
        if f == 's' {
            i = 1;
        }
        while abs(diff) > atol {
            diff = (neg(i) as f64) * pow(x,i) / (fact(i) as f64);
            var += diff;
            i += 2;
        }
        var
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

    fn neg(i: u8) -> i8 {
        match i % 4 {
            0 | 1 => 1,
            2 | 3 => -1,
            _ => 1
        }
    }
}
