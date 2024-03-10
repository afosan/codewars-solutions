//! https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

fn sum_of_digits(n: i64) -> i64 {
    let mut n = n;
    let mut s = 0_i64;

    while n > 0 {
        s += n % 10;
        n /= 10;
    }

    s
}

pub fn digital_root(n: i64) -> i64 {
    let mut n = n * n.signum();

    while n >= 10 {
        n = sum_of_digits(n);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
    }    
}
