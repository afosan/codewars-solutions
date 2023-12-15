//! https://www.codewars.com/kata/563b662a59afc2b5120000c6/train/rust

pub fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut i = 0;
    let mut p_curr = p0;
    
    while p_curr < p {
        i += 1;
        p_curr += (p_curr as f64 * percent / 100f64).floor() as i32 + aug;
    }
    
    i
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans =nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        
    }
}
