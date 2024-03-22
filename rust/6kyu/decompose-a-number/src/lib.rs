//! https://www.codewars.com/kata/55ec80d40d5de30631000025/train/rust

pub fn decompose(n: u32) -> (Vec<u32>, u32)  {
    let mut n = n as f64;
    let mut c = 2_f64;
    let mut out = vec![];
    
    loop {
        let l = n.log(c) as u32;
        
        if l <= 1 {
            break;
        }
        
        out.push(l);
        n -= c.powf(l as f64);
        c += 1_f64;
    }
    
    (out, n as u32)
}

#[cfg(test)]
mod tests {
    use super::decompose;

    #[test]
    fn sample_tests() {
        assert_eq!(decompose(0), (vec![], 0));
        assert_eq!(decompose(4), (vec![2], 0));
        assert_eq!(decompose(9), (vec![3], 1));
        assert_eq!(decompose(25), (vec![4, 2], 0));
        assert_eq!(decompose(8330475), (vec![22, 13, 10, 8, 7, 6, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2], 0));
    }
}
