//! https://www.codewars.com/kata/5899aa695401a83a5c0000c4/train/rust

pub fn square_area_to_circle(size: f64) -> f64 {
    size * std::f64::consts::PI / 4_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a:f64, b:f64, epsilon:f64) {
        assert!( (a-b).abs() < epsilon, "Expected: {}, got: {}",b,a);
    }
    
    #[test]
    fn returns_expected() {
        assert_close(square_area_to_circle(9.0), 7.0685834705770345, 1e-8);
        assert_close(square_area_to_circle(20.0), 15.70796326794897, 1e-8);
    }
}
