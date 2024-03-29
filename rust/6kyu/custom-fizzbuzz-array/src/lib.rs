//! https://www.codewars.com/kata/5355a811a93a501adf000ab7/train/rust

#[macro_export]
macro_rules! fizz_buzz_custom {
    () => { fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5) };
    ($str_one:expr) => { fizz_buzz_custom_solver($str_one, "Buzz", 3, 5) };
    ($str_one:expr, $str_two:expr) => { fizz_buzz_custom_solver($str_one, $str_two, 3, 5) };
    ($str_one:expr, $str_two:expr, $num_one:expr) => { fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5) };
    ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => { fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two) };
}

pub fn fizz_buzz_custom_solver(string_one: &str, string_two: &str, num_one: usize, num_two: usize) -> Vec<String> {
    (1..=100).map(|i| {
        let mut v = vec![];

        if i % num_one == 0 { v.push(string_one.to_owned()); }
        if i % num_two == 0 { v.push(string_two.to_owned()); }

        if v.len() > 0 { v.into_iter().collect::<String>() }
        else { format!("{i}") }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(fizz_buzz_custom!()[3], "4".to_string());
        assert_eq!(fizz_buzz_custom!()[15], "16".to_string());
    }
}
