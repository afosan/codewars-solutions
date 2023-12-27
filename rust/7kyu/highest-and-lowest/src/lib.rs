//! https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust

pub fn high_and_low(numbers: &str) -> String {
    let (minn, maxx) = numbers.split_whitespace().map(|w| w.parse::<i32>().expect("str parsable into i32")).fold(
        (i32::MAX, i32::MIN),
        |(mut acc_min, mut acc_max), n| {
            if n < acc_min {
                acc_min = n;
            }
            if n > acc_max {
                acc_max = n;
            }
            (acc_min, acc_max)
        }
    );

    format!("{} {}", maxx, minn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_1() {
      assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }
    
    #[test]
    fn example_test_2() {
      assert_eq!("3 1", high_and_low("1 2 3"));
    }    
}
