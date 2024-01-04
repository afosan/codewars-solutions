//! https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust

pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        _ => true,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}
