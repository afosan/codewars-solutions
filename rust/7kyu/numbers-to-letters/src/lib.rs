//! https://www.codewars.com/kata/57ebaa8f7b45ef590c00000c/train/rust

pub fn switcher(numbers: Vec<&str>) -> String {
    numbers.iter().map(|s| match s.parse::<u32>().expect("cannot parse str into u32") {
        n @ 1..=26 => char::from_u32((26 - n) + 'a' as u32).unwrap(),
        27 => '!',
        28 => '?',
        29 => ' ',
        _ => panic!("unexpected str"),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::switcher;

    #[test]
    fn example_tests() {
        assert_eq!(switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]), "codewars");
        assert_eq!(switcher(vec!["25","7","8","4","14","23","8","25","23","29","16","16","4"]), "btswmdsbd kkw"); 
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
