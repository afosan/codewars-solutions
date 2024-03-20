//! https://www.codewars.com/kata/5274d9d3ebc3030802000165/train/rust

fn gcd(x: u16, y: u16) -> u16 {
    let (mi, mx) = (x.min(y), x.max(y));
    
    if mi == 0 { mx } else { gcd(mi, mx % mi) }
}

pub fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let g = gcd(x, y);
    (y / g, x / g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(nbr_of_laps(5, 3), (3, 5));
        assert_eq!(nbr_of_laps(4, 6), (3, 2));
        assert_eq!(nbr_of_laps(5, 5), (1, 1));
    }
}
