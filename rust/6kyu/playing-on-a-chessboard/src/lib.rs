//! https://www.codewars.com/kata/55ab4f980f2d576c070000f4/train/rust

pub fn game(n: u64) -> Vec<u64> {
    let sq = n * n;
    if n % 2 == 0 {
        vec![sq / 2]
    } else {
        vec![sq, 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: Vec<u64>) -> () {
        assert_eq!(game(n), exp)
    }
    
    #[test]
    fn basics_game() {
    
        testing(204, vec![20808]);
        testing(807, vec![651249, 2]);
        testing(5014, vec![12570098]);
        testing(750001, vec![562501500001, 2]);
    
    }    
}
