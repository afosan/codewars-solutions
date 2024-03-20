//! https://www.codewars.com/kata/5a626fc7fd56cb63c300008c/train/rust

use regex::Regex;

pub fn uncollapse(digits: &str) -> String {
    let r = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let w = r.captures_iter(digits).map(|c| c.get(0).unwrap().as_str()).collect::<Vec<_>>();
    
    w.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(uncollapse("three"), "three".to_string());
        assert_eq!(uncollapse("eightsix"), "eight six".to_string());
        assert_eq!(uncollapse("fivefourseven"), "five four seven".to_string());
        assert_eq!(uncollapse("ninethreesixthree"), "nine three six three".to_string());
        assert_eq!(uncollapse("foursixeighttwofive"), "four six eight two five".to_string());
    }
}
