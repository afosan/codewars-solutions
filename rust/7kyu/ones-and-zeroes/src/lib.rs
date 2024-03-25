//! https://www.codewars.com/kata/650a86e8404241005fc744ca/train/rust

pub fn same_length(txt: &str) -> bool {
    let mut s = 0;
    let mut is_prev_one = false;
    
    for c in txt.chars() {
        match c {
            '1' => {
                if !is_prev_one && s > 0 {
                    return false;
                }
                is_prev_one = true;
                s += 1;
            },
            '0' => {
                is_prev_one = false;
                s -= 1;
                if s < 0 {
                    return false;
                }
            },
            _ => panic!("unexpected char"),
        }
    }
    
    s == 0
}

#[cfg(test)]
mod tests {
    use super::same_length;
        
    fn dotest(txt: &str, expected: bool) {
        let actual = same_length(txt);
        assert!(actual == expected, 
            "With txt = \"{txt}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("0", false);
        dotest("10", true);
        dotest("1010", true);
        dotest("1001", false);
        dotest("101", false);
        dotest("110010", true);
        dotest("10010", false);
        dotest("110", false);
        dotest("11001", false);
        dotest("1011100010", true);
        dotest("11100011000", false);
        dotest("11101010010010", false);
        dotest("00110100001111", false);
        dotest("1100111000100", false);
        dotest("00110011100010", false);
    }
}
