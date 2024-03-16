//! https://www.codewars.com/kata/557e8a141ca1f4caa70000a6/train/rust

pub fn is_triangle_number(n: u64) -> bool {
    // x**2 + x - 2 * n = 0
    let det = 1 + 8 * n;
    
    (det as f64).sqrt().fract() == 0_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(is_triangle_number(3), true);
        assert_eq!(is_triangle_number(5), false);
        assert_eq!(is_triangle_number(8), false);
        assert_eq!(is_triangle_number(10), true);
        assert_eq!(is_triangle_number(20), false);
    }
    
    #[test]
    fn test_zero_and_one() {
        assert_eq!(is_triangle_number(0), true);
        assert_eq!(is_triangle_number(1), true);        
    }
}
