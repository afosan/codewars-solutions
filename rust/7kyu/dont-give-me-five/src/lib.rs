//! https://www.codewars.com/kata/5813d19765d81c592200001a/train/rust

fn to_digits(n: isize) -> Vec<u8> {
    let mut n = n.abs() as u64;

    if n == 0 {
        vec![0]
    } else {
        let mut out = vec![];

        while n > 0 {
            out.push((n % 10).try_into().unwrap());
            n /= 10;
        }

        out
    }
}

pub fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end).filter(|&n| to_digits(n).into_iter().find(|&d| d == 5).is_none()).count() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(dont_give_me_five(1, 9), 8);
        assert_eq!(dont_give_me_five(4, 17), 12);
    }
}
