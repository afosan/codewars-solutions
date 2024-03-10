//! https://www.codewars.com/kata/5287e858c6b5a9678200083c/train/rust

fn to_digits(n: u64) -> Vec<u64> {
    if n == 0 { return vec![0] };

    let mut n = n;
    let mut digits = Vec::<u64>::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

pub fn narcissistic(num: u64) -> bool {
    let digits = to_digits(num);
    let l = digits.len() as u32;

    digits.iter().map(|d| d.pow(l)).sum::<u64>() == num
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(actual, expected, "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}", input)
    }

    #[test]
    fn basic_tests() {
        dotest(   7,  true);
        dotest( 371,  true);
        dotest( 122, false);
        dotest(4887, false);
    }
}
