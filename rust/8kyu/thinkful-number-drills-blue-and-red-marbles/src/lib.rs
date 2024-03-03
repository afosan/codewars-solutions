//! https://www.codewars.com/kata/5862f663b4e9d6f12b00003b/train/rust

pub fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    let br = (blue_start - blue_pulled) as f64;
    let rr = (red_start - red_pulled) as f64;
    let prob = br / (br + rr);
    
    prob as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(guess_blue(5, 5, 2, 3), 0.6);
        assert_eq!(guess_blue(5, 7, 4, 3), 0.2);
        assert_eq!(guess_blue(12, 18, 4, 6), 0.4);
    }
}
