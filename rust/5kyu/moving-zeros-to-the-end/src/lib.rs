//! https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust

pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(arr.len());
    let mut zero_count = 0;

    for a in arr {
        if *a == 0 {
            zero_count += 1;
        } else {
            out.push(*a);
        }
    }

    out.append(&mut vec![0u8; zero_count]);

    out
}

#[cfg(test)]
mod tests {
    use super::move_zeros;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")   
    }
    
    #[test]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}
