//! https://www.codewars.com/kata/51e0007c1f9378fa810002a9/train/rust

pub fn parse(code: &str) -> Vec<i32> {
    let mut v = 0_i32;
    let mut out = vec![];
    
    code.chars().for_each(|c| match c {
        'i' => { v += 1; },
        'd' => { v -= 1; },
        's' => { v *= v; },
        'o' => { out.push(v); },
        _ => {},
    });
    
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}
