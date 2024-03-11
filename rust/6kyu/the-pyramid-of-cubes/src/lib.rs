//! https://www.codewars.com/kata/61707b71059070003793bc0f/train/rust

pub fn find_height (cubes: usize) -> u16 {
    (0..).take_while(|n| n * (n + 1) * (n + 2) / 6 <= cubes).last().unwrap() as u16 
}

#[cfg(test)]
mod tests {
    use super::find_height;

    #[test]
    fn sample_tests() {
        assert_eq!(find_height(0), 0);
        assert_eq!(find_height(1), 1);
        assert_eq!(find_height(3), 1);
        assert_eq!(find_height(4), 2);
        assert_eq!(find_height(10), 3);
    }
}
