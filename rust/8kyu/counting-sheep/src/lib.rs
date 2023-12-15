//! https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust

pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&v| v).count() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_sheep_count() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);
    }
}
