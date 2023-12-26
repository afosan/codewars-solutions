//! https://www.codewars.com/kata/5a87449ab1710171300000fd/train/rust

pub fn tidy_number(n: u64) -> bool {
    let mut num = n;
    let mut last_digit = n % 10;

    while num > 9 {
        num /= 10;
        let d = num % 10;

        if d > last_digit {
            return false;
        }

        last_digit = d;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }
}
