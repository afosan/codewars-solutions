//! https://www.codewars.com/kata/65127141a5de2b1dcb40927e/train/rust

pub fn spin_around(lst: &[&str]) -> u32 {
    let v = lst.iter().map(|w| match *w {
        "left" => 1,
        "right" => -1,
        _ => 0,
    }).sum::<i32>();

    v.abs() as u32 / 4
}

#[cfg(test)]
mod tests {
    use super::spin_around;
        
    fn dotest(lst: &[&str], expected: u32) {
        let actual = spin_around(lst);
        assert!(actual == expected, 
            "With lst = {lst:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&["left", "right", "left", "right"], 0);
        dotest(&["right", "right", "right", "right", "right", "right", "right", "right"], 2);
        dotest(&["left", "left", "left", "left"], 1);
        dotest(&[], 0);
        dotest(&["left"], 0);
        dotest(&["right"], 0);
        dotest(&["right", "right", "right", "left", "right", "right"], 1);
        dotest(&["left", "left", "right", "left", "left", "left", "left", "left", "left", "right", "left", "left", "right", "right", "right", "right", "left", "left", "right", "right"], 1);
        dotest(&["right", "left", "left", "right", "left", "left", "right", "left", "right", "right", "left", "left", "right", "right", "right", "left", "left", "right"], 0);
        dotest(&["right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right", "right"], 10);
        dotest(&["left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left", "left"], 10);
    }
}
