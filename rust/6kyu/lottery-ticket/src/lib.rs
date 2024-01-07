//! https://www.codewars.com/kata/57f625992f4d53c24200070e/train/rust

pub fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    if ticket.iter().filter(|(s, num)| s.as_ref().chars().any(|c| c as u64 == *num as u64)).count() >= win {
        "Winner!"
    } else {
        "Loser!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2), "Loser!");
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1), "Winner!");
        assert_eq!(bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3), "Loser!");
    }
}
