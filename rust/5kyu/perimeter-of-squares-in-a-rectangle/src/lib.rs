//! https://www.codewars.com/kata/559a28007caad2ac4e000083/train/rust

pub fn perimeter(n: u64) -> u64 {
    let mut sum = 0;
    let (mut f0, mut f1) = (1, 1);
    
    for _ in 0..=n {
        sum += f0;
        (f0, f1) = (f1, f0 + f1);
    }
    
    4 * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }
    
    #[test]
    fn basics_perimeter() {
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}
