//! https://www.codewars.com/kata/5a3dd29055519e23ec000074/train/rust

pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a.iter().zip(arr_b.iter()).map(|(&a, &b)| match (a, b) {
        (_, "") => 0,
        (_, _) if a == b => 4,
        _ => -1,
    }).sum::<i64>().max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b",  ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["",  "a", "a", "c"]), 0);
    }
}
