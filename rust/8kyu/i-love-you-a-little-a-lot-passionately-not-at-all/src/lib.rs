//! https://www.codewars.com/kata/57f24e6a18e9fad8eb000296/train/rust

pub fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    let texts = [
        "I love you",
        "a little",
        "a lot",
        "passionately",
        "madly",
        "not at all",
    ];
    
    texts[(nb_petals as usize - 1) % texts.len()]
}

#[cfg(test)]
mod tests {
    use super::how_much_i_love_you;

    #[test]
    fn fixed_tests() {
        assert_eq!(how_much_i_love_you(7), "I love you");
        assert_eq!(how_much_i_love_you(3), "a lot");
        assert_eq!(how_much_i_love_you(6), "not at all");
    }
}
