//! https://www.codewars.com/kata/62a611067274990047f431a8/train/rust

pub fn alternate<'a>(n: usize, first_value: &'a str, second_value: &'a str) -> Vec<&'a str> {
    (0..n).map(|i| if i % 2 == 0 { first_value } else { second_value }).collect()
}

#[cfg(test)]
mod tests {
    use super::alternate;

    #[test]
    fn sample_tests() {
        assert_eq!(alternate(5, "true", "false"), ["true", "false", "true", "false", "true"]);
        assert_eq!(alternate(20, "blue", "red"), ["blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red", "blue", "red"]);
        let empty_vec:Vec<String> = Vec::new();
        assert_eq!(alternate(0, "lemons", "apples"), empty_vec);
    }
}
