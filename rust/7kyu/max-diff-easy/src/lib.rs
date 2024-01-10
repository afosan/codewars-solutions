//! https://www.codewars.com/kata/588a3c3ef0fbc9c8e1000095/train/rust

pub fn max_diff(numbers: &[i32]) -> i32 {
    if numbers.len() <= 1 {
        0
    } else {
        let (minn, maxx) = numbers.into_iter().fold((i32::MAX, i32::MIN), |(acc_min, acc_max), &n| {
            (acc_min.min(n), acc_max.max(n))
        });

        maxx - minn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 6]), 6);
    }
    
    #[test]
    fn test_sample_2() {
        assert_eq!(max_diff(&[-0, 1, 2, -3, 4, 5, -6]), 11);
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 16]), 16);
    }
    
    #[test]
    fn test_sample_4() {
        assert_eq!(max_diff(&[16]), 0);
    }
    
    #[test]
    fn test_sample_5() {
        assert_eq!(max_diff(&[]), 0);
    }
}
