//! https://www.codewars.com/kata/5a4d303f880385399b000001/train/rust

pub fn strong(n: u64) -> String {
    let mut num = n;
    let mut sum = 0;

    while num > 0 {
        let d = num % 10;
        num /= 10;
        sum += (1..=d).product::<u64>();
    }

    if sum == n {
        "STRONG!!!!"
    } else {
        "Not Strong !!"
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");
        
        // // Testing for 2
        // assert_eq!(strong(2), "STRONG!!!!");
        
        // // Testing for 145
        // assert_eq!(strong(145), "STRONG!!!!");
        
        // // Testing for 7
        // assert_eq!(strong(7), "Not Strong !!");
        
        // // Testing for 93
        // assert_eq!(strong(93), "Not Strong !!");
        
        // // Testing for 185
        // assert_eq!(strong(185), "Not Strong !!");
    }
}
