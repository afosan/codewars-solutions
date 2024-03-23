//! https://www.codewars.com/kata/59b8614a5227dd64dc000008/train/rust

pub fn min_distance(n: u32) -> u32 {
    let factors = (1..=n).filter(|i| n % i == 0).collect::<Vec<_>>();
    factors.windows(2).map(|p| p[1] - p[0]).min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::min_distance;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(n: u32, expected: u32) {
        assert_eq!(min_distance(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn fixed_tests() {
        dotest(8, 1);
        dotest(1, 0);
        dotest(13013, 2);
        dotest(44969, 40);
        dotest(4453, 12);
    }
}
