use std::collections::HashSet;

fn is_happy(year: u16) -> bool {
    let s = year.to_string();
    s.chars().collect::<HashSet<_>>().len() == s.len()
}

pub fn next_happy_year(year: u16) -> u16 {
    (year+1..).skip_while(|&n| !is_happy(n)).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(next_happy_year(1001), 1023);
        assert_eq!(next_happy_year(1123), 1203);
        assert_eq!(next_happy_year(2001), 2013);
        assert_eq!(next_happy_year(2334), 2340);
        assert_eq!(next_happy_year(3331), 3401);
        assert_eq!(next_happy_year(1987), 2013);
        assert_eq!(next_happy_year(5555), 5601);
        assert_eq!(next_happy_year(7712), 7801);
        assert_eq!(next_happy_year(8088), 8091);
        assert_eq!(next_happy_year(8999), 9012);
    }
}
