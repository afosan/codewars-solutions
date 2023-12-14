//! https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust

pub fn max_sequence(seq: &[i32]) -> i32 {
    seq.iter().fold((0, 0), |(curr_sum, max_sum), &n| {
            let new_curr_sum = i32::max(0, curr_sum + n);
            let new_max_sum = i32::max(max_sum, new_curr_sum);
            (new_curr_sum, new_max_sum)
        }
    ).1
}

#[cfg(test)]
mod tests {
    use super::max_sequence;
    
    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}
