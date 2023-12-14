//! https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust

pub fn tower_builder(n_floors: usize) -> Vec<String> {
    (1..=n_floors).map(|n| format!("{}{}{}", " ".repeat(n_floors - n), "*".repeat(2 * n - 1), " ".repeat(n_floors - n))).collect()
}

#[cfg(test)]
mod tests {
    use super::tower_builder;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
