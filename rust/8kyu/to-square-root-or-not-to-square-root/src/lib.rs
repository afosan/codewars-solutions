//! https://www.codewars.com/kata/57f6ad55cca6e045d2000627/train/rust

pub fn square_or_square_root(arr: &[u32]) -> Vec<u32> {
    arr.iter().map(|&n| {
        let nsqrt = (n as f64).sqrt();

        if nsqrt == nsqrt.floor() {
            nsqrt as u32
        } else {
            n * n
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::square_or_square_root;

    fn dotest(arr: &[u32], expected: &[u32]) {
        let actual = square_or_square_root(arr);
        assert!(actual == expected, "Test failed with  arr = {arr:?}\nExpected {expected:?} but got {actual:?}");
    }

    #[test]
    fn sample_tests() {
        dotest(&[4, 3, 9, 7, 2, 1], &[2, 9, 3, 49, 4, 1]);
        dotest(&[100, 101, 5, 5, 1, 1], &[10, 10201, 25, 25, 1, 1]);
        dotest(&[1, 2, 3, 4, 5, 6], &[1, 4, 9, 2, 25, 36]);
    }
}
