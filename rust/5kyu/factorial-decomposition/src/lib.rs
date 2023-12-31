//! https://www.codewars.com/kata/5a045fee46d843effa000070/train/rust

use std::collections::HashMap;

pub fn decomp(n: i32) -> String {
    let mut pfs = HashMap::<u32, u32>::new();
    
    (2..=n as u32).for_each(|mut v| {
        let mut p = 2;
        while v > 1 {
            while v % p == 0 {
                *pfs.entry(p).or_insert(0) += 1;
                v /= p;
            }
            p += if p == 2 { 1 } else { 2 };
        }
    });

    let mut pfs_vec = pfs.into_iter().collect::<Vec<(u32, u32)>>();
    pfs_vec.sort_by_key(|v| v.0);

    pfs_vec.iter().map(|(p, i)| if *i == 1 { format!("{p}") } else { format!("{p}^{i}") }).collect::<Vec<String>>().join(" * ")
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");

    }    
}
