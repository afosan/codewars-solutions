//! https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/rust

pub fn solution(s: &str) -> Vec<String> {
    let mut s = s.to_string();
    if s.len() % 2 == 1 {
        s.push_str(&"_");
    }

    (0..s.len()).step_by(2).map(|i| s[i..i+2].to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
