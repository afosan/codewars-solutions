//! https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust

pub fn abbrev_name(name: &str) -> String {
    name.split_whitespace().map(|w| w.chars().nth(0).unwrap().to_ascii_uppercase().to_string()).collect::<Vec<_>>().join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(abbrev_name("Sam Harris"), "S.H");
        assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
        assert_eq!(abbrev_name("Evan Cole"), "E.C");
        assert_eq!(abbrev_name("P Favuzzi"), "P.F");
        assert_eq!(abbrev_name("David Mendieta"), "D.M");
    }
}
