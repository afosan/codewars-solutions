//! https://www.codewars.com/kata/58235a167a8cb37e1a0000db/train/rust

use std::collections::HashMap;

pub fn number_of_pairs(gloves: &[&str]) -> u32 {
    let mut cs = HashMap::<&str, u32>::new();
    
    gloves.iter().for_each(|s| *cs.entry(s).or_insert(0) += 1);
    
    cs.values().map(|c| c / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(arr: &[&str], expected: u32) {
        assert_eq!(number_of_pairs(arr), expected, "{ERR_MSG} with gloves = {arr:?}")   
    }

    #[test]
    fn fixed_tests() {
        dotest(&["red","red"], 1);
        dotest(&["red","green","blue"], 0);
        dotest(&["gray","black","purple","purple","gray","black"], 3);
        dotest(&[], 0);
        dotest(&["red","green","blue","blue","red","green","red","red","red"], 4);
    }
}
