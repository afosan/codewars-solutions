//! https://www.codewars.com/kata/5edc8c53d7cede0032eb6029/train/rust

pub fn solve(n: u64) -> Option<u64> {    
    let b_some = (1..=n).skip_while(|i| i * i <= n || n % i != 0 || (n % 2 == 0 && (i % 2 == 1 || (n / i) % 2 == 1))).next();
    if b_some.is_none() { return None; }
    let b = b_some.unwrap();
    let s = n / b;
    let diff = b - s;
    
    if diff > 0 && diff % 2 == 0 {
        Some( (diff / 2).pow(2) )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_tests() {
        assert_eq!(solve(1), None);
        assert_eq!(solve(2), None);
        assert_eq!(solve(3), Some(1));
        assert_eq!(solve(4), None);
        assert_eq!(solve(5), Some(4));
        assert_eq!(solve(7), Some(9));
        assert_eq!(solve(8), Some(1));    
        assert_eq!(solve(9), Some(16));
        assert_eq!(solve(10), None);
        assert_eq!(solve(11), Some(25));
        assert_eq!(solve(13), Some(36));
        assert_eq!(solve(17), Some(64));
        assert_eq!(solve(88901), Some(5428900));
        assert_eq!(solve(290101), Some(429235524));
    }
}
