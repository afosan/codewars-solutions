//! https://www.codewars.com/kata/587731fda577b3d1b0001196/train/rust

pub fn camel_case(str: &str) -> String {
    str
        .split_whitespace()
        .map(|w|
            format!("{}{}", &w[0..1].to_uppercase(), &w[1..].to_lowercase())
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }
}
