//! https://www.codewars.com/kata/5b752a42b11814b09c00005d/train/rust

pub fn solve(a: usize, b: usize) -> (usize, usize){
    if a == 0 || b == 0 {
        (a, b)
    } else if a >= 2 * b {
        solve(a - 2 * b, b)
    } else if b >= 2 * a {
        solve(a, b - 2 * a)
    } else {
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_tests() {
        assert_eq!(solve(6, 19), (6, 7));
        assert_eq!(solve(2, 1), (0, 1));
        assert_eq!(solve(22, 5), (0, 1));
        assert_eq!(solve(2, 10), (2, 2));
        assert_eq!(solve(8796203, 7556), (1019, 1442));
        assert_eq!(solve(19394, 19394), (19394, 19394));
    }
}
