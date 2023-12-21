//! https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust

pub fn solution(s: &str) -> String {
    let mut out = Vec::with_capacity(2 * s.len());

    for (c0, c1) in s.chars().zip(s.chars().skip(1)) {
        match (c0, c1) {
            ('a'..='z', 'A'..='Z') => {
                out.push(c0);
                out.push(' ');
            },
            (_, _) => {
                out.push(c0);
            }
        }
    }

    out.into_iter().chain(s.chars().last()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
