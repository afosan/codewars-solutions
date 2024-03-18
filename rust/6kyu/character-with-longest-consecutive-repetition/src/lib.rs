//! https://www.codewars.com/kata/586d6cefbcc21eed7a001155/train/rust

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    let mut pc: Option<char> = None;
    let mut ps = 0;
    let mut out: Option<(char, usize)> = None;
    
    s
        .chars()
        .for_each(|c|
            if pc.is_none() || pc.unwrap() == c {
                ps += 1;
                pc = Some(c);
                
                if out.is_none() || out.unwrap().1 < ps {
                    out = Some((c, ps));
                }
            } else {
                ps = 1;
                pc = Some(c);
            }
        );
    
    out
}

#[cfg(test)]
mod tests {
    use super::longest_repetition;

    #[test]
    fn longest_at_the_beginning() {
        assert_eq!(longest_repetition(&"aaaabbb"), Some(('a', 4)));
    }
    
    #[test]
    fn longest_at_the_end() {
        assert_eq!(longest_repetition(&"abbbbb"), Some(('b', 5)));
        assert_eq!(longest_repetition(&"bbbaaabaaaa"), Some(('a', 4)));
    }
    
    #[test]
    fn longest_in_the_middle() {
        assert_eq!(longest_repetition(&"cbdeuuu900"), Some(('u', 3)));
    }
    
    #[test]
    fn multiple_longest() {
        assert_eq!(longest_repetition(&"aabb"), Some(('a', 2)));
        assert_eq!(longest_repetition(&"ba"), Some(('b', 1)));
    }
    
    #[test]
    fn single_character_only() {
        assert_eq!(longest_repetition(&"aaaaaa"), Some(('a', 6)));
    }
    
    #[test]
    fn empty_string() {
        assert_eq!(longest_repetition(&""), None);
    }
}
