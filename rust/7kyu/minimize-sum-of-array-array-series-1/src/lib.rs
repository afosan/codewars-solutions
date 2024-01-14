//! https://www.codewars.com/kata/5a523566b3bfa84c2e00010b/train/rust

pub fn min_sum(xs: &[u64]) -> u64 {
    let l = xs.len();
    let mut v = xs.iter().collect::<Vec<_>>();
    v.sort_unstable();

    (0..l/2).map(|i| v[i] * v[l - 1 - i]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5,4,2,3]), 22);
        assert_eq!(min_sum(&[12,6,10,26,3,24]), 342);
        assert_eq!(min_sum(&[9,2,8,7,5,4,0,6]), 74);        
    }        
}
