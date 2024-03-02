//! https://www.codewars.com/kata/563a631f7cbbc236cf0000c2/train/rust

pub fn move_hero(position: u32, roll: u32) -> u32 {
    position + 2 * roll
}

#[cfg(test)]
mod tests {
    use super::move_hero;
    
    #[test]
    fn sample_tests() {
        assert_eq!(move_hero(0, 4), 8);
    }
}
