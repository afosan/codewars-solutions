//! https://www.codewars.com/kata/55e785dfcb59864f200000d9/train/rust

fn first_n_primes(n: u8) -> Vec<u64> {
    let mut ps = vec![];
    let mut i = 2;

    while ps.len() < n as usize {
        if ps.iter().all(|p| i % p != 0) {
            ps.push(i);
        }
        
        i += 1;
    }

    ps
}

pub fn count_spec_mult(n: u8, max_val: u64) -> u64 {
    let ps = first_n_primes(n);
    let f = ps.iter().product::<u64>();
    
    max_val / f
}

#[cfg(test)]
mod tests {
    use super::count_spec_mult;

    #[test]
    fn fixed_tests() {
        assert_eq!(count_spec_mult(3, 200), 6);
        assert_eq!(count_spec_mult(3, 1000), 33);
        assert_eq!(count_spec_mult(4, 500), 2);
        assert_eq!(count_spec_mult(4, 1000), 4);
    }
}
