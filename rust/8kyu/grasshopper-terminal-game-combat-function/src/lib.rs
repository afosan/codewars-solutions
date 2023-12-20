//! https://www.codewars.com/kata/586c1cf4b98de0399300001d/train/rust

pub fn combat(health: f32, damage: f32) -> f32 {
    (health - damage).max(0f32) 
}

#[cfg(test)]
mod tests {
    use super::combat;
    
    #[test]
    fn example_tests() {
        assert_eq!(combat(100.0, 5.0), 95.0);
        assert_eq!(combat(92.0, 8.0), 84.0);
        assert_eq!(combat(20.0, 30.0), 0.0, "Health cannot go below 0");
    }
}
