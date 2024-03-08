//! https://www.codewars.com/kata/64ef24b0679cdc004d08169e/train/rust

pub fn min_turns(current: &str, target: &str) -> u8 {
    current.chars().zip(target.chars()).map(
        |(c1, c2)| {
            let n1 = c1.to_digit(10).expect("cannot parse char to u8") as u8;
            let n2 = c2.to_digit(10).expect("cannot parse char to u8") as u8;
            
            let diff = if n1 >= n2 { n1 - n2 } else { n2 - n1 };
            
            diff.min(10 - diff)
        }
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::min_turns;
        
    fn dotest(current: &str, target: &str, expected: u8) {
        let actual = min_turns(current, target);
        assert!(actual == expected, 
            "With current = \"{current}\", target = \"{target}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("4089", "5672", 9);
        dotest("1732", "4444", 9);
        dotest("7109", "2332", 13);
        dotest("2391", "4984", 10);
        dotest("1234", "3456", 8);
        dotest("1111", "1100", 2);
        dotest("1111", "0000", 4);
        dotest("0000", "9999", 4);
    }
    
}
