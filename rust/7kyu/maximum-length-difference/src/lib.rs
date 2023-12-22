//! https://www.codewars.com/kata/5663f5305102699bad000056/train/rust

fn minmax(ss: Vec<&str>) -> (usize, usize) {
    ss.iter().map(|s| s.len()).fold((usize::MAX, usize::MIN), |(mut acc_min, mut acc_max), n| {
        if n < acc_min {
            acc_min = n;
        }
        if n > acc_max {
            acc_max = n;
        }

        (acc_min, acc_max)
    })
}

pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.len() == 0 || a2.len() == 0 {
        return -1;
    }

    let (a1min, a1max) = minmax(a1);
    let (a2min, a2max) = minmax(a2);

    (a2max as i32 - a1min as i32).max(a1max as i32 - a2min as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec!["hoqq", "bbllkw", "oox", "ejjuyyy", "plmiis", "xxxzgpsssa", "xxwwkktt", "znnnnfqknaz", "qqquuhii", "dvvvwz"];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}
