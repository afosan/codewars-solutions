//! https://www.codewars.com/kata/595bbea8a930ac0b91000130/train/rust

pub fn calculate_1_rm(w: i32, r: i32) -> i32 {
    if r == 0 {
        return 0;
    }

    if r == 1 {
        return w;
    }    
    
    let w = w as f64;
    let r = r as f64;
    
    let c1 = (w * (1_f64 + r / 30_f64)).round() as i32;
    let c2 = (100_f64 * w / (101.3_f64 - 2.67123_f64 * r)).round() as i32;
    let c3 = (w * r.powf(0.1_f64)).round() as i32;
    
    return c1.max(c2).max(c3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(calculate_1_rm(135, 20), 282);
        assert_eq!(calculate_1_rm(200, 8), 253);
        assert_eq!(calculate_1_rm(270, 2), 289);
        assert_eq!(calculate_1_rm(360, 1), 360);
        assert_eq!(calculate_1_rm(400, 0), 0);
    }
}
