//! https://www.codewars.com/kata/5f7c38eb54307c002a2b8cc8/train/rust

pub fn remove_parentheses(s: &str) -> String {
    let mut cnt = 0u32;

    s.chars().filter(|&c| {
        if c == '(' {
            cnt += 1;
        } else if c == ')' {
            if cnt == 0 {
                return true;
            }
            if cnt == 1 {
                cnt -= 1;
                return false;
            }

            cnt -= 1;
        }

        cnt == 0
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::remove_parentheses;

    #[test]
    fn sample_tests() {
        assert_eq!(remove_parentheses("example(unwanted thing)example"), "exampleexample");
        assert_eq!(remove_parentheses("example (unwanted thing) example"), "example  example");
        assert_eq!(remove_parentheses("a (bc d)e"), "a e");
        assert_eq!(remove_parentheses("a(b(c))"), "a");
        assert_eq!(remove_parentheses("hello example (words(more words) here) something"), "hello example  something");
        assert_eq!(remove_parentheses("(first group) (second group) (third group)"), "  ");
    }
}
