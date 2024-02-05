//! https://www.codewars.com/kata/602db3215c22df000e8544f0/train/rust

pub fn two_are_positive(a: i32, b: i32, c: i32) -> bool {
    vec![a, b, c].iter().map(|i| if i.is_positive() { 1 } else { 0 }).sum::<u64>() == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_solution(2, 4, -3, true);
        assert_solution(0, 6, 8, true);
        assert_solution(4, -6, 9, true);
        assert_solution(-4, 6, 0, false);
        assert_solution(4, 6, 10, false);
        assert_solution(-14, -3, -4, false);
    }

    // Helper function to provide a more informative assert message.
    fn assert_solution(a: i32, b: i32, c: i32, expected: bool) {
        assert!(
            two_are_positive(a, b, c) == expected,
            "\nexpected: two_are_positive({a}, {b}, {c}) == {expected}\n"
        );
    }
}
