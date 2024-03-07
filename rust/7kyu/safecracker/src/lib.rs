//! https://www.codewars.com/kata/6501aa820038a6b0bd098afb/train/rust

pub fn safecracker(start: u8, incs: &(u16, u16, u16)) -> (u8, u8, u8) {
    let m = 100;
    let start = start as u16;
    let n1 = (m + start - (incs.0 % m)) % m;
    let n2 = (n1 + (incs.1 % m)) % m;
    let n3 = (m + n2 - (incs.2 % m)) % m;
    
    (n1 as u8, n2 as u8, n3 as u8)
}

#[cfg(test)]
mod tests {
    use super::safecracker;
        
    fn dotest(start: u8, incs: &(u16, u16, u16), expected: (u8, u8, u8)) {
        let actual = safecracker(start, incs);
        assert!(actual == expected, 
            "With start = {start}, incs = {incs:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        for (start, incs, expected) in [
            (5,  &(100,200,100),   (5, 5, 5)),
            (99, &(87, 61, 91),    (12, 73, 82)),
            (63, &(22, 16, 35),    (41, 57, 22)),
            (18, &(10, 57, 96),    (8, 65, 69)),
            (83, &(37, 12, 7),     (46, 58, 51)),
            (31, &(44, 86, 23),    (87, 73, 50)),
            (96, &(47, 76, 89),    (49, 25, 36)),
            (82, &(41, 5, 66),     (41, 46, 80)),
            (31, &(59, 79, 99),    (72, 51, 52)),
            (95, &(14, 35, 34),    (81, 16, 82)),
            (77, &(73, 98, 55),    (4, 2, 47)),
            (78, &(86, 4, 40),     (92, 96, 56)),
            (0,  &(21, 94, 92),    (79, 73, 81)),
            (45, &(63, 96, 30),    (82, 78, 48)),
            (98, &(25, 93, 94),    (73, 66, 72)),
            (27, &(64, 25, 63),    (63, 88, 25)),
            (81, &(272, 244, 200), (9, 53, 53)),
            (99, &(999, 998, 997), (0, 98, 1)),
            (57, &(944, 819, 796), (13, 32, 36)),
            (56, &(777, 722, 943), (79, 1, 58)),
            (70, &(806, 818, 766), (64, 82, 16)),
        ]
        {
            dotest(start, incs, expected);
        }
    }
}
