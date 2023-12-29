//! https://www.codewars.com/kata/5b180e9fedaa564a7000009a/train/rust

pub fn solve(s: &str) -> String {
    let diff_lower_minus_upper = s.chars().map(|c| match c {
        'a'..='z' => 1,
        'A'..='Z' => -1,
        _ => 0,
    }).sum::<i32>();

    if diff_lower_minus_upper < 0 {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
