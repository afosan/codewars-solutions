//! https://www.codewars.com/kata/57ae18c6e298a7a6d5000c7a/train/rust

pub fn replace_all<T: PartialEq + Copy>(xs: &[T], find: T, replace: T) -> Vec<T> {
    xs.iter().map(|x| if *x == find { replace } else { *x }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(replace_all(&[], 1, 2), []);
        assert_eq!(replace_all(&[1, 2, 2], 1, 2), [2, 2, 2]);
        assert_eq!(replace_all(&["ooh", "la", "la"], "la", "baby"), ["ooh", "baby", "baby"])
    }
}
