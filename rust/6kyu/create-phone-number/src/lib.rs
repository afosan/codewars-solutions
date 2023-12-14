//! https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust

pub fn create_phone_number(numbers: &[u8]) -> String {
    let numbers_as_string = numbers.iter().map(|&n| char::from_digit(n as u32, 10).expect("expected a digit base 10")).collect::<String>();
    format!("({}) {}-{}", &numbers_as_string[..3], &numbers_as_string[3..6], &numbers_as_string[6..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
        assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
      }
}
