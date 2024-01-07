//! https://www.codewars.com/kata/5a54e796b3bfa8932c0000ed/train/rust

pub fn jumping_number(n: u64) -> String {
    let mut curr = n % 10;
    let mut num = n / 10;

    while num > 0 {
        let next = num % 10;
        if next != curr + 1 && next + 1 != curr {
            return "Not!!".to_string();
        }
        curr = next;
        num /= 10;
    }

    return "Jumping!!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(jumping_number(1), "Jumping!!");
        assert_eq!(jumping_number(7), "Jumping!!");
        assert_eq!(jumping_number(9), "Jumping!!");
        assert_eq!(jumping_number(23), "Jumping!!");
        assert_eq!(jumping_number(32), "Jumping!!");
        assert_eq!(jumping_number(79), "Not!!");
        assert_eq!(jumping_number(98), "Jumping!!");
        assert_eq!(jumping_number(8987), "Jumping!!");
        assert_eq!(jumping_number(4343456), "Jumping!!");
        assert_eq!(jumping_number(98789876), "Jumping!!");
    }
}
