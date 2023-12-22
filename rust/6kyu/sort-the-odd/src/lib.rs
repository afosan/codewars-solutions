//! https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds = arr.iter().filter(|n| *n % 2 == 1).collect::<Vec<_>>();
    odds.sort();
    let mut i = 0;

    arr.iter().map(|n| if *n % 2 == 0 { *n } else { i += 1; *odds[i - 1] }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
