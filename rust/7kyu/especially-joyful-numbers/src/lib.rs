//! https://www.codewars.com/kata/570523c146edc287a50014b1/train/rust

fn digitize(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }
    
    let mut digits = Vec::<u32>::new();
    let mut num = n;

    while num > 0 {
        let d = num % 10;
        num /= 10;

        digits.push(d);
    }

    digits
}

pub fn number_joy(n: u32) -> bool {
    let sum_of_digits = digitize(n).iter().sum();
    let digits = digitize(sum_of_digits);
    let n1 = digits.iter().fold(0u32, |acc, i| 10 * acc + i);
    let n2 = digits.iter().rev().fold(0u32, |acc, i| 10 * acc + i);

    n1 * n2 == n
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn do_test(n: u32, expected: bool, msg: &str) {
        assert_eq!(number_joy(n), expected, "{n}: {msg}");
    }
    
    #[test]
    fn test() {
        do_test(1997, false, "Not a Harshad number");
        do_test(1998, false, "Harshad but digit sum=27, and 27x72 does not equal 1998");
        do_test(1729, true, "Harshad and digit sum=19, and 19x91 = 1729");
        do_test(18, false, "Harshad but digit sum=9, and 9x9 does not equal 18");
        do_test(1, true, "Is a Harshad number");
        do_test(81, true, "Is a Harshad number");
        do_test(1458, true, "Is a Harshad number");
    }
}
