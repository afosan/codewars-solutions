//! https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust

pub fn to_csv_text(array: &[Vec<i8>]) -> String {
    array.iter().map(|row| row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::to_csv_text;
    
    fn do_test(input: &[Vec<i8>], expected: &str) {
        let actual  = to_csv_text(input);
        assert!(actual == expected,
            "Test failed with array = {input:?}\nExpected \"{expected}\"\nbut got \"{actual}\"");
    }

    #[test]
    fn fixed_tests() {
        for (input, expected) in [
            (vec![
            vec![0, 1, 2, 3, 45],
            vec![10, 11, 12, 13, 14],
            vec![20, 21, 22, 23, 24],
            vec![30, 31, 32, 33, 34]
        ], "0,1,2,3,45\n10,11,12,13,14\n20,21,22,23,24\n30,31,32,33,34"),
            (vec![
            vec![-25, 21, 2, -33, 48],
            vec![30, 31, -32, 33, -34]
        ], "-25,21,2,-33,48\n30,31,-32,33,-34"),
            (vec![
            vec![5, 55, 5, 5, 55],
            vec![6, 6, 66, 23, 24],
            vec![127, 31, 66, 33, 7]
        ], "5,55,5,5,55\n6,6,66,23,24\n127,31,66,33,7")
        ] {
            do_test(&input, expected)
        }
    }
}
