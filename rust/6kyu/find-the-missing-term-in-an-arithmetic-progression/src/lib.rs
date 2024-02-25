//! https://www.codewars.com/kata/52de553ebb55d1fca3000371/train/rust

pub fn find_missing(seq: &[i32]) -> i32 {
    let l = seq.len() as i32;
    let first = seq.iter().nth(0).unwrap();
    let last = seq.iter().last().unwrap();
    let diff = (last - first) / l;
    let sm = seq.iter().sum::<i32>();
    
    (l + 1) * first + l * (l + 1) * diff / 2 - sm
}

#[cfg(test)]
mod tests {
    use super::find_missing;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[i32], expected: i32) {
        assert_eq!(find_missing(a), expected, "{ERR_MSG} with seq = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 6, 7, 8, 9], 5);
        dotest(&[1, 3, 4, 5, 6, 7, 8, 9], 2);
    }
}
