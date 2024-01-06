//! https://www.codewars.com/kata/53c9157c689f841d16000c03/train/rust

fn totient (n: u64) -> usize {
    let mut n = n;
    let mut tot = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            tot -= tot / i;
        }

        while n % i == 0 {
            n /= i;
        }

        i += 1;
    }

    (if n > 1 {
        tot - tot / n
    } else {
        tot
    }) as usize
}

#[cfg(test)]
mod tests {
    use super::totient;

    #[test]
    fn sample_tests() {
        assert_eq!(totient(1), 1);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(9), 6);
        assert_eq!(totient(983), 982);
        assert_eq!(totient(1601), 1600);
        assert_eq!(totient(9999999985), 7849056384);
    }
}
