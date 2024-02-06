//! https://www.codewars.com/kata/583ade15666df5a64e000058/train/rust

pub fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 { format!("{n:b}") } else { format!("{n:x}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(evens_and_odds(2),"10");
        assert_eq!(evens_and_odds(0),"0");
        assert_eq!(evens_and_odds(13),"d");
        assert_eq!(evens_and_odds(47),"2f");
        assert_eq!(evens_and_odds(12800),"11001000000000");
        assert_eq!(evens_and_odds(8172381723),"1e71ca61b");
    }
}
