//! https://www.codewars.com/kata/5ce6728c939bf80029988b57/train/rust

pub fn solve(s: &str) -> bool {
    let mut v = s.chars().collect::<Vec<_>>();
    v.sort_unstable();
    
    v.windows(2).all(|pair| pair[0] as u32 + 1 == pair[1] as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(solve("abc"), true);
        assert_eq!(solve("abd"), false);
        assert_eq!(solve("dabc"), true);
        assert_eq!(solve("abbc"), false);
    }
}
