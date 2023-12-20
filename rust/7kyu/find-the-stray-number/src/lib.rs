//! https://www.codewars.com/kata/57f609022f4d534f05000024/train/rust

pub fn stray(arr: &[u32]) -> u32 {
    match arr {
        [a, b, rest @ ..] if a == b => *rest.iter().find(|n| *n != a).unwrap(),
        [a, b, c, ..] => {
            if a == c {
                *b
            } else if b == c {
                *a
            } else {
                unreachable!();
            }
        },
        _ => {
            panic!("expected arr to have at least 3 elements");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::stray;
        
    fn dotest(arr: &[u32], expected: u32) {
        let actual = stray(arr);
        assert!(actual == expected, 
            "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 1, 1, 1, 1, 1, 2], 2);
        dotest(&[2, 3, 2, 2, 2], 3);
        dotest(&[3, 2, 2, 2, 2], 3);
    }
}
