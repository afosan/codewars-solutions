//! https://www.codewars.com/kata/54da539698b8a2ad76000228/train/rust

pub fn is_valid_walk(walk: &[char]) -> bool {
    walk.len() == 10 &&
    walk.iter().fold(
        (0i64, 0i64),
        |(acc0, acc1), c| match c {
            'n' => (acc0 + 1, acc1),
            's' => (acc0 - 1, acc1),
            'w' => (acc0, acc1 + 1),
            'e' => (acc0, acc1 - 1),
            _ => panic!("unexpected char"),
        }
    ) == (0, 0)
}

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(  is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        assert!(! is_valid_walk(&['w']));
        assert!(! is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }
}
