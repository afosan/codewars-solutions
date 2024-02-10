//! https://www.codewars.com/kata/5ba178be875de960a6000187/train/rust

use std::collections::HashMap;

fn count_chars(n: u64) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    n.to_string().chars().for_each(|c| *counts.entry(c).or_insert(0) += 1);

    counts
}

fn sorted_str(n: u64) -> String {
    let mut s = n.to_string().chars().collect::<Vec<char>>();
    s.sort();

    s.iter().collect()
}

pub fn find_lowest_int(k: u64) -> u64 {
    let mut i = 1;
    let mut n1 = k;
    let mut n2 = k + 1;

    while count_chars(n1) != count_chars(n2) {
        i += 1;
        n1 += k;
        n2 += k + 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::find_lowest_int;
        
    fn dotest(n: u64, expected: u64) {
        let actual = find_lowest_int(n);
        assert!(actual == expected, 
            "With k = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(325,477);
        dotest(599,2394);
        dotest(855, 999);
        dotest(1,125874);
        dotest(100,8919);
        dotest(1000,89919);
        dotest(10000,899919);
    }
}
