//! https://www.codewars.com/kata/5727bb0fe81185ae62000ae3/train/rust

pub fn clean_string(s: &str) -> String {
    let mut v = vec![];
    
    s.chars().for_each(|c| {
        match c {
            '#' => { v.pop(); },
            _ => v.push(c),
        }
    });
    
    v.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(clean_string("abc#d##c"), "ac");
        assert_eq!(clean_string("abc####d##c#"), "");
        assert_eq!(clean_string("#######"), "");
        assert_eq!(clean_string(""), "");
    }
}
