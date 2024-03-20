//! https://www.codewars.com/kata/59c3e8c9f5d5e40cab000ca6/train/rust

pub fn add_arrays(arr_a: &[i64], arr_b: &[i64]) -> Vec<i64> {
    if arr_a.len() == 0 {
        return arr_b.to_vec();
    }
    if arr_b.len() == 0 {
        return arr_a.to_vec();
    }
    
    let a = arr_a.iter().map(|i| i.to_string()).collect::<String>().parse::<i64>().expect("cannot parse to i64");
    let b = arr_b.iter().map(|i| i.to_string()).collect::<String>().parse::<i64>().expect("cannot parse to i64");
    let s = a + b;
    
    s.abs().to_string().chars().enumerate().map(|(i, c)| {
        let d = c.to_digit(10).expect("cannot convert") as i64;
        
        if i == 0 { d * s.signum() } else { d }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(add_arrays(&[6, 7], &[6, 7]), [1, 3, 4]);
        assert_eq!(add_arrays(&[3, 2, 9], &[1, 2]), [3, 4, 1]);
        assert_eq!(add_arrays(&[4, 7, 3], &[1, 2, 3]), [5, 9, 6]);
        assert_eq!(add_arrays(&[1], &[5, 7, 6]), [5, 7, 7]);
        assert_eq!(add_arrays(&[-3, 4, 2], &[3, 4, 4]), [2]);
        assert_eq!(add_arrays(&[], &[]), []);
        assert_eq!(add_arrays(&[0], &[]), [0]);
        assert_eq!(add_arrays(&[], &[1, 2]), [1, 2]);
    }
}
