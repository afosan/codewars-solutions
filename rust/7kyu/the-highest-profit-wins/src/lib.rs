//! https://www.codewars.com/kata/559590633066759614000063/train/rust

pub fn min_max(lst: &[i32]) -> (i32, i32) {
    lst.iter().fold((i32::MAX, i32::MIN), |(mut min_so_far, mut max_so_far), &n| {
        if n < min_so_far {
            min_so_far = n;
        }
        if n > max_so_far {
            max_so_far = n;
        }

        (min_so_far, max_so_far)
    })
}

#[cfg(test)]
mod tests {
    use super::min_max;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected: (i32, i32)) {
        assert_eq!(min_max(arr), expected, "{ERR_MSG} with lst = {arr:?}")
    }

    #[test]
    fn fixed_tests() {
        for (arr, expected) in [(vec![1,2,3,4,5], (1,5)), (vec![2334454,5], (5, 2334454))] {
            dotest(&arr, expected)
        }
    }
}
