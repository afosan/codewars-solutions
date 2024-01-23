//! https://www.codewars.com/kata/55685cd7ad70877c23000102/train/rust

pub fn make_negative(number: i32) -> i32 {
    if number > 0 {
        -number
    } else {
        number
    }
}

#[cfg(test)]
mod tests {
    use super::make_negative;
    
    #[test]
    fn sample_tests() {
        assert_eq!(make_negative(1), -1);
        assert_eq!(make_negative(-5), -5);
        assert_eq!(make_negative(0), 0);
    }
}
