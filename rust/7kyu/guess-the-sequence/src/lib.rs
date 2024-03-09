//! https://www.codewars.com/kata/5b45e4b3f41dd36bf9000090/train/rust

pub fn sequence(x: u8) -> Vec<u8> {
    let mut v = (1..=x).collect::<Vec<u8>>();
    
    v.sort_by_key(|i| i.to_string());
    
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            sequence(16),
            [1, 10, 11, 12, 13, 14, 15, 16, 2, 3, 4, 5, 6, 7, 8, 9],
            "sequence(16)",
        );
        
        assert_eq!(
            sequence(9),
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            "sequence(9)",
        );
    }
}
