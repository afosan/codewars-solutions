//! https://www.codewars.com/kata/5966eeb31b229e44eb00007a/train/rust

pub fn vaporcode(s: &str) -> String {
    s.to_uppercase().chars().filter(|c| *c != ' ').map(|c| c.to_string()).collect::<Vec<_>>().join("  ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(vaporcode("Lets go to the movies"), "L  E  T  S  G  O  T  O  T  H  E  M  O  V  I  E  S".to_string());
        assert_eq!(vaporcode("Why isn't my code working?"), "W  H  Y  I  S  N  '  T  M  Y  C  O  D  E  W  O  R  K  I  N  G  ?".to_string());
    }
}
