//! https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/rust

pub fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|(i, s)| format!("{}: {}", i + 1, s)).collect()
}

#[cfg(test)]
mod tests {
    use super::number;

    fn dotest(arr: &[&str], expected: &[&str]) {
        let actual = number(arr);
        assert!(actual == expected, 
            "With lines= {arr:?}\nExpected {expected:?}\nBut got {actual:?}")
    }
    
    #[test]
    fn sample_tests() {
        dotest(&[], &[]);
        dotest(&["a", "b", "c"], &["1: a", "2: b", "3: c"]);
        dotest(&["", "", ""], &["1: ", "2: ", "3: "]);
        dotest(&["", "b", "", ""], &["1: ", "2: b", "3: ", "4: "]);
    }
}
