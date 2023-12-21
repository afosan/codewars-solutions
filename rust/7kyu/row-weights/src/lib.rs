//! https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust

pub fn row_weights(array: Vec<u32>) -> (u32, u32) {
    array.iter().enumerate().fold((0u32, 0u32), |(acc0, acc1), (i, &n)| {
        if i % 2 == 0 {
            (acc0 + n, acc1)
        } else {
            (acc0, acc1 + n)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
      assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
      assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
      assert_eq!(row_weights(vec![80]), (80,0));
    }
}
