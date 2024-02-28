//! https://www.codewars.com/kata/570409d3d80ec699af001bf9/train/rust

pub fn fusc(n: u32) -> u32{
    match n {
        0 => 0,
        1 => 1,
        _ if n % 2 == 0 => fusc(n / 2),
        _ => fusc(n / 2) + fusc(n / 2 + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::fusc;

    #[test]
    fn tests() {
        assert_eq!(fusc(0), 0, "\nWith n=0, your answer (left) is not the expected answer (right).");
        assert_eq!(fusc(1), 1, "\nWith n=1, your answer (left) is not the expected answer (right).");
        assert_eq!(fusc(85), 21, "\nWith n=85, your answer (left) is not the expected answer (right).");
    }
}
