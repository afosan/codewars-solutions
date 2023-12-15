//! https://www.codewars.com/kata/55a2d7ebe362935a210000b2/train/rust

pub fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut min_so_far = i32::MAX;
    
    for &n in arr {
        if n < min_so_far {
            min_so_far = n;
        }
    }
    
    min_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}
