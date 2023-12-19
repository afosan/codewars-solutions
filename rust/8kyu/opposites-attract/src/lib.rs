//! https://www.codewars.com/kata/555086d53eac039a2a000083/train/rust

pub fn lovefunc(flower1: u16, flower2: u16) -> bool {
    let m1 = flower1 % 2;
    let m2 = flower2 % 2;

    (m1 == 0 && m2 == 1) ||(m1 == 1 && m2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test(1, 4, true);
        do_test(2, 2, false);
        do_test(0, 1, true);
        do_test(0, 0, false);
    }
    
    fn do_test(f1: u16, f2: u16, exp: bool) {
        assert_eq!(lovefunc(f1, f2), exp, "\nFailed with flower1 {}, flower2 {}", f1, f2);
    }
}
