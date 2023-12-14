//! https://www.codewars.com/kata/5626b561280a42ecc50000d1/train/rust

pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b).filter(|&n| digit_power_sum(n) == n).collect()
}

fn digit_power_sum(n: u64) -> u64 {
    let mut sum = 0;
    let mut power = n.to_string().len() as u32;
    let mut n = n;

    while n > 0 {
        sum += (n % 10).pow(power);
        power -= 1;
        n /= 10;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::sum_dig_pow;
        
    fn dotest(a: u64, b: u64, expected: &[u64]) {
        let actual = sum_dig_pow(a, b);
        assert!(actual == expected, "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(1, 10, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        dotest(1, 100, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 89]);
        dotest(10, 89, &[89]);
        dotest(10, 100, &[89]);
        dotest(90, 100, &[]);
        dotest(89, 135, &[89, 135]);
    }
}
