//! https://www.codewars.com/kata/52b5247074ea613a09000164/train/rust

pub fn cooking_time(eggs: u32) -> u32 {
    5 * ((eggs + 1) / 8 + 1 - if eggs % 8 == 0 { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        do_test(0, 0);
        do_test(5, 5);
        do_test(10, 10);
    }
    
    fn do_test(eggs: u32, exp: u32) {
        let user_time = cooking_time(eggs);
        assert!(
            user_time == exp,
            "Test failed!\ncooking_time({}) was {}\nExpected {}",
            eggs,
            user_time,
            exp,
        );
    }
}
