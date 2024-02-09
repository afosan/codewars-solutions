//! https://www.codewars.com/kata/5506b230a11c0aeab3000c1f/train/rust

pub fn evaporator(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let evap = 1_f64 - (evap_per_day as f64 / 100_f64);
    let thres = threshold as f64 / 100_f64;
    let mut i = 0_i32;
    let mut rem = 1_f64;
    
    while rem > thres {
        i += 1;
        rem *= evap;
    }
    
    i
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
        
    }
}
