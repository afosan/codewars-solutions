//! https://www.codewars.com/kata/5629db57620258aa9d000014/train/rust

use std::collections::HashMap;
use std::cmp::{Ordering, Reverse};
use itertools::Itertools;

fn count_lowercase_letters(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    s
        .chars()
        .filter(|c| *c >= 'a' && *c <= 'z')
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    counts
}

pub fn mix(s1: &str, s2: &str) -> String {
    let s1_counts = count_lowercase_letters(s1);
    let s2_counts = count_lowercase_letters(s2);

    ('a'..='z')
        .map(|c| {
            let s1_count = s1_counts.get(&c).unwrap_or(&0usize);
            let s2_count = s2_counts.get(&c).unwrap_or(&0usize);

            let ordering = match s1_count.cmp(&s2_count) {
                Ordering::Less => '2',
                Ordering::Greater => '1',
                Ordering::Equal => '=',
            };

            (c, s1_count.max(s2_count), ordering)
        })
        .filter(|(_, m, _)| **m > 1)
        .sorted_by_key(|t| (Reverse(t.1), t.2, t.0))
        .map(|(c, m, o)| format!("{}:{}", o, c.to_string().repeat(*m)))
        .collect::<Vec<String>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::mix;
    
    #[test]
    fn basics_mix() {
        testing("Are they here", "yes, they are here", 
            "2:eeeee/2:yy/=:hh/=:rr");
        testing("looping is fun but dangerous", "less dangerous than coding", 
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        testing(" In many languages", " there's a pair of functions", 
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing("A generation must confront the looming ", "codewarrs", 
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    }
    
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}
