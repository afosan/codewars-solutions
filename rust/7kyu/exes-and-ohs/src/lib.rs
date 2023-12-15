//! https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust

pub fn xo(string: &'static str) -> bool {
    string.chars().map(|c| match c {
        'x' | 'X' => 1,
        'o' | 'O' => -1,
        _ => 0,
    }).sum::<i64>() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(xo("xo"), true);
        assert_eq!(xo("Xo"), true);
        assert_eq!(xo("xxOo"), true);
        assert_eq!(xo("xxxm"), false);
        assert_eq!(xo("Oo"), false);
        assert_eq!(xo("ooom"), false);
    }
}
