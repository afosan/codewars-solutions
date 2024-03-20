//! https://www.codewars.com/kata/55031bba8cba40ada90011c4/train/rust

use regex::Regex;

pub fn is_sum_of_cubes(s: &str) -> String {
    let re = Regex::new(r"(\d{1,3})").unwrap();
    let nums = re.captures_iter(s).map(|c| c[0].parse::<u64>().unwrap()).collect::<Vec<_>>();
    let cubics = nums.iter().filter(|&&n| {
        let d0 = (n % 10).pow(3);
        let d1 = ((n / 10) % 10).pow(3);
        let d2 = ((n / 100) % 10).pow(3);
        
        d0 + d1 + d2 == n
    }).copied().collect::<Vec<_>>();

    if cubics.len() == 0 {
        "Unlucky".to_string()
    } else {
        let s1 = cubics.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ");
        let s2 = cubics.iter().sum::<u64>();
        format!("{s1} {s2} Lucky")
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        println!("s: {:?};", s);
        let ans = is_sum_of_cubes(s);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("00 9026315 -827&()", "0 0 Lucky");
        dotest("0 9026315 -827&()", "0 0 Lucky");
        dotest("Once upon a midnight dreary, while100 I pondered, 9026315weak and weary -827&()", "Unlucky"); 
        
    }
}
