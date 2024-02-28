//! https://www.codewars.com/kata/6411b91a5e71b915d237332d/train/rust

pub fn valid_parentheses(parens: &str) -> bool {
    let mut cnt = 0_u32;
    
    for c in parens.chars() {
        if c == '(' {
            cnt += 1;
        } else if c == ')' {
            if cnt == 0 {
                return false;
            }
            cnt -= 1;
        }
    }
    
    cnt == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn do_test(expected: bool, input: &str) {
        assert_eq!(valid_parentheses(input), expected, "\nYour result (left) did not match the expected output (right) for the input: {input:?}");
    }

    #[test]
    fn valid_cases() {
        do_test(true, "()");
        do_test(true, "((()))");
        do_test(true, "()()()");
        do_test(true, "(()())()");
        do_test(true, "()(())((()))(())()");
    }
    
    #[test]
    fn invalid_cases()  {
        do_test(false, ")(");
        do_test(false, "()()(");
        do_test(false, "((())");
        do_test(false, "())(()");
        do_test(false, ")()");
        do_test(false, ")");
    }
    
    #[test]
    fn empty_string() {
        do_test(true, "");
    }
}
