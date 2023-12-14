//! https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec/train/rust

pub fn persistence(num: u64) -> u64 {
    let mut count = 0;
    let mut n = num;

    while n >= 10 {
        count += 1;
        n = digit_multiply(n);
    }

    count
}

fn digit_multiply(num: u64) -> u64 {
    if num < 10 {
        num
    } else {
        let mut prod = 1;
        let mut n = num;

        while n > 0 {
            prod *= n % 10;
            n /= 10;
        }

        prod
    }
}

#[cfg(test)]
mod tests {
    use super::persistence;

    #[test]
    fn sample_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(25), 2);
        assert_eq!(persistence(999), 4);
    }
}
