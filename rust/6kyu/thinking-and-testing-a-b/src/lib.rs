//! https://www.codewars.com/kata/5a90c9ecb171012b47000077/train/rust

fn sum_of_digits(n: u64) -> u64 {
    let mut s = 0;
    let mut n = n;
    
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    
    s
}

pub fn test_it(a: u64, b: u64) -> u64 {
    sum_of_digits(a) * sum_of_digits(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Hmm.. 0 * 1 = 0
        assert_eq!(test_it(0, 1), 0, "Testing for: a=0 b=1");

        // Yes, 1 * 2 = 2
        assert_eq!(test_it(1, 2), 2, "Testing for: a=1 b=2");

        // I know, 5 * 6 = 30
        assert_eq!(test_it(5, 6), 30, "Testing for: a=5 b=6");

        // What? 10 * 10 = 1 ?
        assert_eq!(test_it(10, 10), 1, "Testing for: a=10 b=10");

        // Damn.. 200 * 200 = 4, 0 was omitted ?
        assert_eq!(test_it(200, 200), 4, "Testing for: a=200 b=200");

        // Discover the mysteries of it ;-)
        assert_eq!(test_it(12, 34), 21, "Testing for: a=12 b=34");

        // You can solve it
        assert_eq!(test_it(123, 45), 54, "Testing for: a=123 b=45");

        // And click ATTEMPT for more challenge..
    }
}
