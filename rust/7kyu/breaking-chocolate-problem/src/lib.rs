//! https://www.codewars.com/kata/534ea96ebb17181947000ada/train/rust

fn nbreak(n: u32) -> u64 {
    if n <= 1 {
        0
    } else {
        format!("{n:b}").chars().rev().enumerate().map(|(i, c)| if c == '1' { i as u64 + 1 } else { 0 }).sum::<u64>()
    }
}

pub fn break_chocolate(n: u32, m: u32) -> u64 {
    let ni = nbreak(n);
    let mi = nbreak(m);

    (ni + n as u64 * mi).min(mi + m as u64 * ni)
}

#[cfg(test)]
mod tests {
    use super::break_chocolate;
        
    fn dotest(n: u32, m: u32, expected: u64) {
        let actual = break_chocolate(n, m);
        assert!(actual == expected, 
            "With n = {n}, m = {m}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(5, 5, 24);
        dotest(7, 4, 27);
        dotest(1, 1, 0);
        dotest(0, 0, 0);
        dotest(6, 1, 5);
    }
}
