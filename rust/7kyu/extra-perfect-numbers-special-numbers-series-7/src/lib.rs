//! https://www.codewars.com/kata/5a662a02e626c54e87000123/train/rust

fn is_extra_perfect(n: u32) -> bool {
    let s = format!("{n:b}");

    s.starts_with("1") && s.ends_with("1")
}

pub fn extra_perfect(n: u32) -> Vec<u32> {
    (1..=n).filter(|i| is_extra_perfect(*i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(extra_perfect(3), [1,3]);
        assert_eq!(extra_perfect(5), [1,3,5]);
        assert_eq!(extra_perfect(7), [1,3,5,7]);
        assert_eq!(extra_perfect(28), [1,3,5,7,9,11,13,15,17,19,21,23,25,27]);
        assert_eq!(extra_perfect(39), [1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31,33,35,37,39]);        
    }
}
