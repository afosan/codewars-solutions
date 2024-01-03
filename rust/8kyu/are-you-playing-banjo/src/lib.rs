//! https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust

pub fn are_you_playing_banjo(name: &str) -> String {
    if let Some(c) = name.chars().nth(0) {
        if c == 'r' || c == 'R' {
            return format!("{name} plays banjo");
        }
    }

    return format!("{name} does not play banjo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
