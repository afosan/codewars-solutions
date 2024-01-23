//! https://www.codewars.com/kata/5f55ecd770692e001484af7d/train/rust

pub fn mirror(list: &[i32]) -> Vec<i32> {
    let mut v = list.into_iter().collect::<Vec<_>>();
    v.sort();

    v.iter().chain(v.iter().rev().skip(1)).copied().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(mirror(&[]), &[]);
        assert_eq!(mirror(&[1]), &[1]);
        assert_eq!(mirror(&[2, 1]), &[1, 2, 1]);
        assert_eq!(mirror(&[2, 3, 1]), &[1, 2, 3, 2, 1]);
        assert_eq!(mirror(&[-8, 42, 18, 0, -16]), &[-16, -8, 0, 18, 42, 18, 0, -8, -16]);
        assert_eq!(mirror(&[-5, 10, 8, 10, 2, -3, 10]), &[-5, -3, 2, 8, 10, 10, 10, 10, 10, 8, 2, -3, -5]);
    }
}
