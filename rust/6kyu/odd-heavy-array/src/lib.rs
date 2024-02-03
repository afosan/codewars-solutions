//! https://www.codewars.com/kata/59c7e477dcc40500f50005c7/train/rust

pub fn is_odd_heavy(arr: &[i32]) -> bool {
    if let Some(&min_odd) = arr.iter().filter(|a| a.rem_euclid(2) == 1).min() {
        arr.iter().filter(|a| a.rem_euclid(2) == 0).all(|a| *a < min_odd)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_odd_heavy;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[i32], expected: bool) {
        assert_eq!(is_odd_heavy(a), expected, "{ERR_MSG} with arr = {a:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[0, 2, 19, 4, 4], true);
        dotest(&[1, -2, -1, 2], false);
        dotest(&[-3, 2, 1, 3, -1, -2], false);
        dotest(&[3, 4, -2, -3, -2], false);
        dotest(&[-1, 1, -2, 2, -2, -2, -4, 4], false);
        dotest(&[-1, -2, 21], true);
        dotest(&[0, 0, 0, 0], false);
        dotest(&[0, -1, 1], false);
        dotest(&[0, 2, 3], true);
        dotest(&[0], false);
        dotest(&[], false);
        dotest(&[1], true);
        dotest(&[0, 1, 2, 3, 4, 0, -2, -1, -4, -3], false);
        dotest(&[1, -1, 3, -1], true);
        dotest(&[1, -1, 2, -2, 3, -3, 0], false);
        dotest(&[3], true);
        dotest(&[2, 4, 6], false);
        dotest(&[-2, -4, -6, -8, -11], false);
    }
}
