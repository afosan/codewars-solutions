//! https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust

pub fn balanced_num(n: u64) -> String {
    let mut v = Vec::<u64>::new();
    let mut num = n;
    
    while num > 0 {
        let d = num % 10;
        num /= 10;
        v.push(d);
    }
    
    let l = v.len();
    let m = l / 2;

    if &v[..if l % 2 == 0 { m - 1 } else { m }].iter().sum::<u64>() == &v[m+1..].iter().sum::<u64>() {
        "Balanced"
    } else {
        "Not Balanced"
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }
    
    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");                
    }    
}
