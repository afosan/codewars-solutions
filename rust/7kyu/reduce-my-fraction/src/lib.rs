//! https://www.codewars.com/kata/576400f2f716ca816d001614/train/rust

fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 || y == 0 {
        x + y
    } else {
        let (minn, maxx) = (x.min(y), x.max(y));
        gcd(minn, maxx % minn)
    }
}

pub fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
    let f = gcd(fraction.0, fraction.1);

    (fraction.0 / f, fraction.1 / f)
}

#[cfg(test)]
mod tests {
    use super::reduce_fraction;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(f: (u32, u32), expected: (u32, u32)) {
        assert_eq!(reduce_fraction(f), expected, "{ERR_MSG} with fraction = {} / {}", f.0, f.1)
    }

    #[test]
    fn simple_fractions() {
        dotest((60, 20), (3, 1));
        dotest((80, 120), (2, 3));
        dotest((4, 2), (2, 1));
        dotest((45, 120), (3, 8));
        dotest((1000, 1), (1000, 1));
        dotest((1, 1), (1, 1));
    }
}
