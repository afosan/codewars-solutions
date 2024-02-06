//! https://www.codewars.com/kata/5648b12ce68d9daa6b000099/train/rust

pub fn number(bus_stops:&[(i32, i32)]) -> i32 {
    bus_stops.iter().map(|(ins, outs)| ins - outs).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
        assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
        assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
    }
}
