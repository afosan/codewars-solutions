//! https://www.codewars.com/kata/5592e3bd57b64d00f3000047/train/rust

pub fn find_nb(m: u64) -> i32 {
    let mut sum = 0;
    let mut i = 0u64;

    while sum < m {
        i += 1;
        sum += i.pow(3);
    }

    if sum == m { i as i32 } else { -1 }
}

#[cfg(test)]
mod sample_tests {
    use super::find_nb;

    fn do_test(n: u64, exp: i32) {
        assert_eq!(find_nb(n), exp, 
                   "\nYour result (left) did not match expected output (right) for n={n}");
    }

    #[test]
    fn basics_find_nb() {
        let cases = [
            (4,               -1),
            (16,              -1),
            (4183059834009,   2022),
            (24723578342962,  -1),
            (135440716410000, 4824),
            (40539911473216,  3568),
            (26825883955641,  3218),
        ];
        for (n, expected) in cases {
            do_test(n, expected);
        }
    }
    
}
