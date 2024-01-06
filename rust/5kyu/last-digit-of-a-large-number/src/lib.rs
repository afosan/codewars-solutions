//! https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }

    let d = str1.chars().last().expect("expected non-empty input").to_digit(10).expect("expected a valid digit");
    let mut remainders = Vec::<u32>::with_capacity(10);
    let mut prev = d;
    remainders.push(prev);

    loop {
        let curr = prev * d % 10;
        if curr == d {
            break;
        }
        remainders.push(curr);
        prev = curr;
    }

    let m = remainders.len();
    let c = str2.chars().fold(0usize, |acc, c| (10 * acc + c.to_digit(10).unwrap() as usize) % m);

    (if c == 0 { remainders[m - 1] } else { remainders[c - 1] }) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(last_digit("4", "1"), 4);
        assert_eq!(last_digit("4", "2"), 6);
        assert_eq!(last_digit("9", "7"), 9);
        assert_eq!(last_digit("10","10000000000"), 0);
        assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
        assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
    }
}
