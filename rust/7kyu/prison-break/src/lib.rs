//! https://www.codewars.com/kata/6507e3170b7009117e0c7865/train/rust

pub fn freed_prisoners(prison: &[bool]) -> u32 {
    let mut target = true;
    let mut c = 0;
    
    for (i, &p) in prison.iter().enumerate() {
        if i == 0 && !p {
            break;
        }
        
        if p == target {
            target = !target;
            c += 1;
        }
    }
    
    c
}

#[cfg(test)]
mod tests {
    use super::freed_prisoners;
        
    fn dotest(prison: &[bool], expected: u32) {
        let actual = freed_prisoners(prison);
        assert!(actual == expected, 
            "With prison = {prison:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[true, true, false, false, false, true, false], 4);
        dotest(&[true, false, false, false, false, false, false], 2);
        dotest(&[true, true, true, false, false, false], 2);
        dotest(&[true, false, true, false, true, false], 6);
        dotest(&[true, true, true], 1);
        dotest(&[false, false, false], 0);
        dotest(&[false, true, true, true], 0);
    }
}
