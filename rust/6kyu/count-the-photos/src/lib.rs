//! https://www.codewars.com/kata/6319dba6d6e2160015a842ed/train/rust

pub fn count_photos(road: &str) -> usize {
    road.chars().fold(
        (0, 0, 0), 
        |(photo, right, point), c| match c {
            '>' => (photo, right + 1, point),
            '.' => (photo + right, right, point + 1),
            '<' => (photo + point, right, point),
            _ => panic!("unexpected char"),
        }
    ).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        let cases = [
            (">.>..<", 8),
            (".><.>>.<<", 11),
            (".>>>", 0),
            (">..<<.>.<.", 15),
            (".<>>..><.<<<<<.", 34),
            ("<..>>..>>.><.<.><..<", 57),
            ("<<.", 0),
            (">>><<<", 0),
            ("..<>.>>.><>>.<<<.<>>.>.>>>>>..><<.>.>>..>.>>><><>.", 248),
        ];
        for (arg, expected) in cases {
            let actual = count_photos(arg);
            assert_eq!(actual, expected, "count_photos(\"{arg}\") should be {expected}, but got {actual}");
        }       
    }

}
