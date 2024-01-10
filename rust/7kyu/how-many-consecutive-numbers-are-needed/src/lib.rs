//! https://www.codewars.com/kata/559cc2d2b802a5c94700000c/train/rust

pub fn consecutive(xs: &[i16]) -> i16 {
    // elements are unique
    let l = xs.len();

    if l <= 1 {
        0
    } else {
        let (minn, maxx) = xs
            .iter()
            .fold((i16::MAX, i16::MIN), |(acc_min, acc_max), &x| {
                (acc_min.min(x), acc_max.max(x))
            });
        
        maxx - minn + 1 - (l as i16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(consecutive(&[4,8,6]), 2);
        assert_eq!(consecutive(&[1,2,3,4]), 0);
        assert_eq!(consecutive(&[]), 0);
        assert_eq!(consecutive(&[1]), 0);
    }
}
