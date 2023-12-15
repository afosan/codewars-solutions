//! https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust

pub fn descending_order(x: u64) -> u64 {
    let mut vec = x.to_string().chars().collect::<Vec<char>>();
    vec.sort();
    vec.iter().rev().collect::<String>().parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
