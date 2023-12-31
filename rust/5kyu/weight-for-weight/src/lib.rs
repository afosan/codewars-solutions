//! https://www.codewars.com/kata/55c6126177c9441a570000cc/train/rust

pub fn order_weight(s: &str) -> String {
    let mut num_strs = s.trim().split_whitespace().collect::<Vec<_>>();
    num_strs.sort_by_key(|num_str| (num_str.chars().map(|c| c.to_digit(10).expect("cannot parse char to digit") as u64).sum::<u64>(), num_str.to_string()));
    num_strs.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s: &str, exp: &str) -> () {
        assert_eq!(order_weight(s), exp)
    }
    
    #[test]
    fn basics_order_weight() {
    
        testing("103 123 4444 99 2000", "2000 103 123 4444 99");
        testing("2000 10003 1234000 44444444 9999 11 11 22 123", 
            "11 11 2000 10003 22 123 1234000 44444444 9999");
        
    }    
}
