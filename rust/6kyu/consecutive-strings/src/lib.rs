//! https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust

pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || k <= 0 || strarr.len() < k {
        return "".into();
    }
    let k_lengths = strarr.iter().map(|s| s.len()).collect::<Vec<_>>().windows(k).map(|ls| ls.iter().sum::<usize>()).collect::<Vec<_>>();
    let max_k_length = k_lengths.iter().max().unwrap();
    let i_start = k_lengths.iter().enumerate().find(|(_, l)| *l == max_k_length).unwrap().0;
    strarr.iter().skip(i_start).take(k).cloned().collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
        assert_eq!(&longest_consec(strarr, k), exp)
    }
    
    #[test]
    fn basics_longest_consec() {
        testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
        testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
            "oocccffuucccjjjkkkjyyyeehh");
        testing(vec![], 3, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    }
}
