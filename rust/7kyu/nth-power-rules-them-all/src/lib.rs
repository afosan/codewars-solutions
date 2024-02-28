//! https://www.codewars.com/kata/58aed2cafab8faca1d000e20/train/rust

pub fn modified_sum(array: &[i32], power: u32) -> i32 {
    let s = array.iter().sum::<i32>();
    let power_s = array.iter().map(|a| a.pow(power)).sum::<i32>();
    
    power_s - s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(modified_sum(&vec![1, 2, 3], 3), 30);
        assert_eq!(modified_sum(&vec![1, 2], 5), 30);
    }
}
