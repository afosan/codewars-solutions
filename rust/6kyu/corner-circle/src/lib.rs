//! https://www.codewars.com/kata/5898761a9c700939ee000011/train/rust

pub fn corner_circle(r: f64) -> f64 {
    let one = 1_f64;
    let sqrt2 = 2_f64.sqrt();
    let res = (sqrt2 - one) / (sqrt2 + one) * r;
    
    let rounding = 100_f64;
    
    (res * rounding).round() / rounding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(corner_circle(3.0), 0.51);
        assert_eq!(corner_circle(17.0), 2.92);
    }
}
