mod preloaded;
use preloaded::NATO;

pub fn to_nato(words: &str) -> String {
    words.to_uppercase().chars().filter(|c| *c != ' ').map(|c| match NATO.get(&c) {
        Some(v) => v.to_string(),
        None => match c {
            ',' | '.' | '!' | '?' => c.to_string(),
            _ => "".to_string(),
        },
    }).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::to_nato;

    #[test]
    fn examples() {
        assert_eq!(
            to_nato("If you can read"),
            "India Foxtrot Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta"
        );
        
        assert_eq!(
            to_nato("Did not see that coming",),
            "Delta India Delta November Oscar Tango Sierra Echo Echo Tango Hotel Alfa Tango Charlie Oscar Mike India November Golf"
        );
        
        assert_eq!(
            to_nato("go for it!"),
            "Golf Oscar Foxtrot Oscar Romeo India Tango !"
        );
    }
}
