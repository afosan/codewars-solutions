//! https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust

pub fn feast(beast: &str, dish: &str) -> bool {
    dish.starts_with(&beast[..1]) && dish.ends_with(&beast[beast.len()-1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(feast("great blue heron", "garlic naan"), true);
        assert_eq!(feast("chickadee", "chocolate cake"), true);
        assert_eq!(feast("brown bear", "bear claw"), false);
    }
}
