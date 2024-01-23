//! https://www.codewars.com/kata/5a91a7c5fd8c061367000002/train/rust

pub fn minimum_steps(nums: &[i32], value: i32) -> usize {
    let mut vs = nums.to_vec();
    vs.sort();

    let mut s = 0;

    for (i, v) in vs.into_iter().enumerate() {
        s += v;
        if s >= value {
            return i;
        }
    }

    unreachable!("expects solution to exist");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(minimum_steps(&[4, 6, 3], 7), 1);
        assert_eq!(minimum_steps(&[10, 9, 9, 8], 17), 1);
        assert_eq!(minimum_steps(&[8, 9, 10, 4, 2], 23), 3);
        assert_eq!(minimum_steps(&[19, 98, 69, 28, 75, 45, 17, 98, 67], 464), 8);
        assert_eq!(minimum_steps(&[4, 6, 3], 2), 0);
    }
}
