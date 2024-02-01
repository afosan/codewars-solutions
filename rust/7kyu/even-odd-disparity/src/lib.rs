//! https://www.codewars.com/kata/59c62f1bdcc40560a2000060/train/rust

pub fn solve(v: &Vec<String>) -> i32 {
    v.iter().fold(0_i32, |acc, s| {
        match s.parse::<i32>() {
            Ok(n) => {
                let d = if n % 2 == 0 { 1 } else { -1 };
                acc + d
            },
            Err(_) => {acc},
        }
    })
}

#[cfg(test)]
mod tests {
    use super::solve;
    
    #[test]
    fn example_tests() {
        let example_inputs = [
            &to_vector(&["0", "1", "2", "3"]),
            &to_vector(&["0","1","2","3","a","b"]),
            &to_vector(&["0","15","z","16","m","13", "14","c", "9", "10", "13","u", "4", "3"]),
            &to_vector(&["5", "15", "16", "10", "6", "4", "16", "t", "13", "n", "14", "k", "n", "0", "q", "d", "7", "9"]),
        ];
        
        assert_eq!(solve(example_inputs[0]), 0);
        assert_eq!(solve(example_inputs[1]), 0);
        assert_eq!(solve(example_inputs[2]), 0);
        assert_eq!(solve(example_inputs[3]), 2);
    }
    
    fn to_vector(strs: &[&'static str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }
}
