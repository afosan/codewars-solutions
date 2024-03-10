//! https://www.codewars.com/kata/5839edaa6754d6fec10000a2/train/rust

pub fn find_missing_letter(chars: &[char]) -> char {
    let first = chars[0] as u64;
    let last = chars[chars.len() - 1] as u64;
    let sum = chars.iter().map(|c| *c as u64).sum::<u64>();

    (
        (first + last) * (last - first + 1) / 2 - sum
    ) as u8 as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
