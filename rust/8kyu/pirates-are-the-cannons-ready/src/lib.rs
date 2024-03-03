//! https://www.codewars.com/kata/5748a883eb737cab000022a6/train/rust

use std::collections::HashMap;

pub fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    let ready = gunners.values().all(|&v| v == "aye");
    
    (if ready { "Fire!" } else { "Shiver me timbers!" }).to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{cannons_ready};
    
    #[test]
    fn test_should_fire_the_cannons_when_ready() {
        assert_eq!(cannons_ready(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "aye"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
            ])), "Fire!");
    }
    
    #[test]
    fn test_should_shiver_your_timbers_instead_if_not_ready() {
        assert_eq!(cannons_ready(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "nay"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
            ])), "Shiver me timbers!");
    }
}
