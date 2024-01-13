//! https://www.codewars.com/kata/59b401e24f98a813f9000026/train/rus

fn from_num_to_digits(n: u16) -> Vec<u8> {
    let mut out = vec![];
    if n == 0 {
        out.push(0);
    } else {
        let mut num = n;
        while num > 0 {
            let d = (num % 10) as u8;
            num /= 10;
            out.push(d);
        }
    }

    out
}

pub fn compute_depth (n: u16) -> u8 {
    let mut exists = vec![false; 10];
    let mut multiple = n;

    while !exists.iter().all(|e| *e) {
        let digits = from_num_to_digits(multiple);
        for d in digits {
            exists[d as usize] = true;
        }
        multiple += n;
    }

    ((multiple - 1) / n) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(compute_depth(1), 10);
        assert_eq!(compute_depth(42), 9);
        assert_eq!(compute_depth(8), 12);
        assert_eq!(compute_depth(13), 8);
        assert_eq!(compute_depth(7), 10);
        assert_eq!(compute_depth(25), 36);
    }
}
