//! https://www.codewars.com/kata/59dd2c38f703c4ae5e000014/train/rust

use regex::Regex;

pub fn solve(s: &str) -> u32 {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(s).map(|w| w.get(1).unwrap().as_str().parse::<u32>().unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(solve(&"gh12cdy695m1"), 695);
        assert_eq!(solve(&"2ti9iei7qhr5"), 9);
        assert_eq!(solve(&"lu1j8qbbb85"), 85);
        assert_eq!(solve(&"185lu1j8qbbb85"), 185);
    }
}
