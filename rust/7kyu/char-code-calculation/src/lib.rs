//! https://www.codewars.com/kata/57f75cc397d62fc93d000059/train/rust

pub fn calc(s: &str) -> u32 {
    let num = s.chars().map(|c| (c as u32).to_string()).collect::<String>();
    num.chars().filter(|c| *c == '7').count() as u32 * 6 
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn example_tests() {
        do_test("ABC", 6);
        do_test("abcdef", 6);
        do_test("ifkhchlhfd", 6);
        do_test("aaaaaddddr", 30);
        do_test("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", 96);
    }

    fn do_test(input: &str, expected: u32) {
        let user_result = calc(input);
        assert!(
            user_result == expected,
            "Test Failed!\ncalc(\"{}\") was {}\nExpected {}",
            input,
            user_result,
            expected
        );
    }
}
