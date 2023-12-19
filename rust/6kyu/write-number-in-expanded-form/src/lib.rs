//! https://www.codewars.com/kata/5842df8ccbd22792a4000245/train/rust

fn digits(n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut ds = vec![];
    let mut n = n;

    while n > 0 {
        let d = (n % 10).try_into().unwrap();
        ds.push(d);
        n /= 10;
    }

    ds
}

pub fn expanded_form(n: u64) -> String {
    digits(n).iter().enumerate().filter(|t| *t.1 != 0).rev().map(|(i, d)| format!("{}{}", d, if i >= 1 { "0".repeat(i) } else { "".to_string() })).collect::<Vec<_>>().join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
