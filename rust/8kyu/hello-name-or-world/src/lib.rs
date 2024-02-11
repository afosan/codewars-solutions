//! https://www.codewars.com/kata/57e3f79c9cb119374600046b/train/rust

pub fn hello(name: &str) -> String {
    let word = if name.len() == 0 {
        "World".to_string()
    } else {
        format!("{}{}", &name[..1].to_uppercase(), &name[1..].to_lowercase())  
    };
    
    format!("Hello, {word}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
