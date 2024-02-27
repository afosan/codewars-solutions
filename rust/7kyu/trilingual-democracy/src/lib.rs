//! https://www.codewars.com/kata/62f17be8356b63006a9899dc/train/rust

pub fn trilingual_democracy(group: &[u8;3]) -> u8 {
    let mut v = group.to_vec();
    v.sort_unstable();
    
    if v[0] == v[1] {
        v[2]
    } else if v[1] == v[2] {
        v[0]
    } else {
        (b"DFIK".iter().map(|c| *c as u64).sum::<u64>() - v.into_iter().map(|i| i as u64).sum::<u64>()) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(group: &[u8;3], lang: u8) {
        let want = lang as char;
        let got = trilingual_democracy(group) as char;
        assert_eq!(want, got, "for group {}: expected {}, got {}", std::str::from_utf8(group).unwrap(), want, got);
    }
    
    #[test]
    fn test_examples() {
        test(b"FFF", b'F');
        test(b"IIK", b'K');
        test(b"DFK", b'I');
    }
}
