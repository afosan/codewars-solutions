pub fn shortest_steps_to_num (n: u16) -> u16 {
    if n == 1 {
        0
    } else if n % 2 == 0 {
        1 + shortest_steps_to_num(n / 2)
    } else {
        1 + shortest_steps_to_num(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::shortest_steps_to_num;

    #[test]
    fn sample_tests() {
        assert_eq!(shortest_steps_to_num(1), 0);
        assert_eq!(shortest_steps_to_num(12), 4);
        assert_eq!(shortest_steps_to_num(16), 4);
        assert_eq!(shortest_steps_to_num(71), 9);
    }
}
