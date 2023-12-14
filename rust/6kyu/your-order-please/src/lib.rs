//! https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust

pub fn order(sentence: &str) -> String {
    let mut vec = sentence
        .split_whitespace()
        .map(|w| (w.chars().find(|c| c.is_digit(10)).expect("no digit found").to_digit(10).unwrap(), w))
        .collect::<Vec<(u32, &str)>>();
    vec.sort_by_key(|t| t.0);
    vec.iter().map(|t| t.1.to_string()).collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
