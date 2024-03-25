//! https://www.codewars.com/kata/5e2596a9ad937f002e510435/train/rust

pub fn infected(s: &str) -> f64 {
    let (i, t) = s.split("X").fold((0_u64, 0_u64), |(i, t), w| {
        let l = w.len() as u64;
        match w.contains("1") {
            true => (i + l, t + l),
            false => (i, t + l),
        }
    });
    
    if t == 0 { 0_f64 } else { i as f64 / t as f64 * 100_f64 }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn approx_equals(sol: f64, exp: f64) {
        assert!((sol-exp).abs() <= 1e-9, "Got {}, expected: {} within a margin of 1e-9", sol, exp);
    }

    #[test]
    fn test_fixed() {
        let tests = vec![("01000000X000X011X0X",73.33333333333333),
                        ("01X000X010X011XX", 72.72727272727273),
                        ("XXXXX", 0.),
                        ("00000000X00X0000", 0.),
                        ("0000000010", 100.),
                        ("000001XXXX0010X1X00010", 100.),
                        ("X00X000000X10X0100",42.857142857142854)];
        tests.into_iter().for_each(|(world, exp)| { approx_equals(infected(world), exp); })
    }
}
