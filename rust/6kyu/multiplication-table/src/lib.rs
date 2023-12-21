//! https://www.codewars.com/kata/534d2f5b5371ecf8d2000a08/train/rust

pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    (1..=len).map(|r| (1..=len).map(|c| r * c).collect::<Vec<_>>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
    }
}
