//! https://www.codewars.com/kata/57f759bb664021a30300007d/train/rust

pub fn switcheroo(s: &str) -> String {
    s.chars().map(|c| match c {
        'a' => 'b',
        'b' => 'a',
        _ => c,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(switcheroo("abc"), "bac");
        assert_eq!(switcheroo("aaabcccbaaa"), "bbbacccabbb");
        assert_eq!(switcheroo("ccccc"), "ccccc");
        assert_eq!(switcheroo("abababababababab"), "babababababababa");
        assert_eq!(switcheroo("aaaaa"), "bbbbb");
    }
}
