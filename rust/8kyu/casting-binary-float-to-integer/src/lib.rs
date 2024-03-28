use std::mem::transmute;

// return binary representation as i32
pub fn convert_to_i32(f: f32) -> i32 {
    unsafe { transmute::<f32, i32>(f) }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn provided_tests() {
        assert_eq!(convert_to_i32(10.0), 1092616192);
        assert_eq!(convert_to_i32(f32::INFINITY), 0x7f800000);
        assert_eq!(convert_to_i32(1.40129846432e-44), 10);
    }
}
