//! https://www.codewars.com/kata/59f4a0acbee84576800000af/train/rust

pub fn pos_average(s: &str) -> f64 {
    let ss = s.split(", ").collect::<Vec<&str>>();
    let l = ss.len();
    let p = (ss[0].len() * l * (l - 1) / 2) as f64;
    let mut c = 0;
    
    for i in 0..l {
        for j in (i+1)..l {
            let s1 = ss[i];
            let s2 = ss[j];
            c += s1.chars().zip(s2.chars()).filter(|x| x.0 == x.1).count() as u64;
            println!("{c}");
        }
    }
    
    (100 * c) as f64 / p
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;
    
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-10;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }
    
    #[test]
    fn basic_tests() {
        assert_float_equals(pos_average("6900690040, 4690606946, 9990494604"), 26.6666666667);
        assert_float_equals(pos_average("466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096"), 26.6666666667);
        assert_float_equals(pos_average("444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490"), 29.2592592593);
    }
}
