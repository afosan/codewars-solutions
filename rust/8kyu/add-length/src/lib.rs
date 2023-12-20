//! https://www.codewars.com/kata/559d2284b5bb6799e9000047/train/rust

pub fn add_length(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| format!("{} {}", w, w.len())).collect()
}

#[cfg(test)]
mod tests {
    use super::add_length;
        
    fn dotest(s: &str, expected: &[String]) {
        let actual = add_length(s);
        assert!(actual == expected, 
            "With s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest("apple ban",&["apple 5", "ban 3"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
        dotest("you will win",&["you 3", "will 4", "win 3"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
        dotest("y",&["y 1"].iter().map(|x| x.to_string()).collect::<Vec<_>>());
    }
}
