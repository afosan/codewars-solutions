//! https://www.codewars.com/kata/52fb87703c1351ebd200081f/train/rust

pub fn what_century(year: &str) -> String {
    let y = year.parse::<u64>().expect("cannot parse");
    let c = (y + 99) / 100;
    let e = match c % 10 {
        _ if 11 <= c && c <= 13 => "th",
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    
    format!("{c}{e}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(what_century("1999"), "20th");
        assert_eq!(what_century("2011"), "21st");
        assert_eq!(what_century("2154"), "22nd");
        assert_eq!(what_century("2259"), "23rd");
        assert_eq!(what_century("1234"), "13th");
        assert_eq!(what_century("1023"), "11th");
        assert_eq!(what_century("2000"), "20th");
        assert_eq!(what_century("3210"), "33rd");        
    }
}
