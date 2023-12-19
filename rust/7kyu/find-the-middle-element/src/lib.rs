//! https://www.codewars.com/kata/545a4c5a61aa4c6916000755/train/rust

pub fn gimme(input_array: [i32;3]) -> usize {
    let i_max = (0..=2).max_by_key(|&i| input_array[i as usize]).unwrap();
    let i_min = (0..=2).min_by_key(|&i| input_array[i as usize]).unwrap();

    (0 + 1 + 2) - i_max - i_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
