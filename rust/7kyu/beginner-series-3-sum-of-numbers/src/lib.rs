//! https://www.codewars.com/kata/55f2b110f61eb01779000053/train/rust

pub fn get_sum(a: i64, b: i64) -> i64 {
    let minn = a.min(b);
    let maxx = a + b - minn;

    (minn..=maxx).sum()
}

#[cfg(test)]
mod tests {
    use super::get_sum;
    
    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
