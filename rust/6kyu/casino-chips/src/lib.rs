//! https://www.codewars.com/kata/5e0b72d2d772160011133654/train/rust

pub fn solve(arr: &[u32; 3]) -> u32 {
    let mut v = arr.iter().collect::<Vec<_>>();
    v.sort_unstable();

    let zero = *v[0];
    let one = *v[1];
    let two = *v[2];
        
    let d = if two - one >= zero { 0 } else { zero + one - two };

    zero + one - d / 2 - if d % 2 == 1 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_cases() {
        assert_eq!(solve(&[1, 1, 1]), 1);
        assert_eq!(solve(&[1, 2, 1]), 2);
        assert_eq!(solve(&[4, 1, 1]), 2);
        assert_eq!(solve(&[8, 2, 8]), 9);
        assert_eq!(solve(&[8, 1, 4]), 5);
        assert_eq!(solve(&[7, 4, 10]), 10);
        assert_eq!(solve(&[12, 12, 12]), 18);
        assert_eq!(solve(&[1, 23, 2]), 3);
    }
}
