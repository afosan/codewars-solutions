//! https://www.codewars.com/kata/55cbd4ba903825f7970000f5/train/rust

pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let avg = (s1 + s2 + s3) as f32 / 3f32;

    if avg >= 90f32 {
        'A'
    } else if avg >= 80f32 {
        'B'
    } else if avg >= 70f32 {
        'C'
    } else if avg >= 60f32 {
        'D'
    } else {
        'F'
    }
}

#[cfg(test)]
mod tests {
    use super::get_grade;

    fn dotest(s1: u16, s2: u16, s3: u16, expected: char) {
        let actual = get_grade(s1, s2, s3);
        assert!(actual == expected, 
            "With s1 = {s1}, s2 = {s2}, s = {s3}\nExpected '{expected}' but got '{actual}'")
    }
    
    #[test]
    fn test_get_grade() {
        dotest(100,100,100, 'A');
        dotest(95,90,93, 'A');
        dotest(82,85,87, 'B');
        dotest(60,82,76, 'C');
        dotest(58,62,70, 'D');
        dotest(58,59,60, 'F');
        dotest(0,0,0, 'F');
    }
}
