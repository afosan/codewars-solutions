//! https://www.codewars.com/kata/5d59576768ba810001f1f8d6/train/rust

pub fn quadratic(x1: i32, x2: i32) -> (i32, i32, i32) {
    let a = 1;
    let b = -a *(x1 + x2);
    let c = a * (x1 * x2);

    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(quadratic(0, 1), (1, -1, 0));
        assert_eq!(quadratic(1, 1), (1, -2, 1));
        assert_eq!(quadratic(-4, -9), (1, 13, 36));
        assert_eq!(quadratic(-5, -4), (1, 9, 20));
        assert_eq!(quadratic(4, -9), (1, 5, -36));
        assert_eq!(quadratic(5, -4), (1, -1, -20));
    }
}
