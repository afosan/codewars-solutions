//! https://www.codewars.com/kata/58e93b4706db4d24ee000096/train/rust

pub fn days_represented(trips: &[(u32, u32)]) -> u32 {
    trips.iter().map(|(s, e)| e - s + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::days_represented;

    #[test]
    fn sample_tests() {
        do_test(&[(10, 15), (25, 35)], 17);
        do_test(&[(2, 8), (220, 229), (10, 16)], 24);
    }
    
    fn do_test(trips: &[(u32, u32)], days: u32) {
        let user_result = days_represented(trips);
        assert!(
            user_result == days,
            "Test failed!\ntrips: {:?}\nGot: {}\nExpected: {}",
            trips,
            user_result,
            days
        );
    }
}
