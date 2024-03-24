//! https://www.codewars.com/kata/51f1342c76b586046800002a/train/rust

pub fn solution(n: f64) -> f64 {
    (2_f64 * n).round() as f64 * 0.5
}

#[cfg(test)]
mod tests {
    use super::solution;
    
    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
    }
}
