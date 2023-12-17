//! https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust

pub fn printer_error(s: &str) -> String {
    let total: usize = s.len();
    let error: usize = s.chars().filter(|c| match c {
        'a'..='m' => false,
        _ => true,
    }).count();

    format!("{error}/{total}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
        assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
    }
}
