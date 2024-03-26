//! https://www.codewars.com/kata/5d653190d94b3b0021ec8f2b/train/rust

use itertools::Itertools;

pub fn product_sum(a: &[u32], m: u32) -> u64 {    
    a.iter().combinations(m as usize).map(|cs| cs.iter().fold(1_u64, |acc, &&c| {
        (acc * c as u64) % (10_u64.pow(9) + 7)
    })).sum()
}

#[cfg(test)]
mod tests {
    use super::product_sum;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], m: u32, expected: u64) {
        assert_eq!(product_sum(a, m), expected, "{ERR_MSG} with a = {a:?}, m = {m}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[3, 10, 7, 9, 1, 4, 5, 2, 8, 6], 7, 8409500);
        dotest(&[10, 7, 8, 5, 6, 9, 4, 1, 2, 3], 8, 12753576);
        dotest(&[1, 7, 6, 10, 21, 5, 9, 8, 5, 4], 2, 2469);
        dotest(&[6, 7, 8, 5, 2, 4, 9, 3, 1, 10], 6, 3416930);
        dotest(&[3, 2, 9, 1, 7, 10, 5, 6, 8, 4], 4, 157773);
        dotest(&[9, 8, 10, 4, 2, 7, 5, 1, 3, 6], 3, 18150);
        dotest(&[3, 3, 1, 7, 6, 8, 1, 4, 6, 10], 4, 94837);
        dotest(&[6, 8, 1, 7, 2, 10, 5, 9, 3, 4], 5, 902055);
        dotest(&[10, 3, 4, 9, 5, 8, 6, 7, 1, 2], 1, 55);
        dotest(&[7, 9, 4, 2, 3, 10, 8, 6, 5, 1], 9, 10628640);
    }
}
