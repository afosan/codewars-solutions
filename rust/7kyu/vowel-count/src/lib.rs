//! https://www.codewars.com/kata/54ff3102c1bad923760001f3/train/rust

pub fn get_count(string: &str) -> usize {
    string.chars().filter(|c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(get_count("abracadabra"), 5);
    }
}
