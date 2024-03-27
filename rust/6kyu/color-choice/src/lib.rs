//! https://www.codewars.com/kata/55be10de92aad5ef28000023/train/rust

fn c(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

pub fn check_choose(m: u64, n: u64) -> i64 {
    if let Some(out) = (0..=n).map(|i| (i, c(n, i))).skip_while(|(_, c_i)| *c_i < m).next() {
        if out.1 == m { out.0 as i64 } else { -1 }
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(m: u64, n: u64, exp: i64) -> () {
        assert_eq!(check_choose(m, n), exp)
    }

    #[test]
    fn basics_check_choose() {
        dotest(6, 4, 2);
        dotest(4, 4, 1);
        dotest(35, 7, 3);
        dotest(36, 7, -1);
        dotest(184756, 20, 10);
    }
}
