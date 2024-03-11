//! https://www.codewars.com/kata/62ca07aaedc75c88fb95ee2f/train/rust

pub fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    (minimum..=maximum).chain((minimum+1..maximum).rev()).map(|n| n.to_string()).cycle().take(length).collect()
}

#[cfg(test)]
mod tests {
    use super::ascend_descend;
    
    fn dotest(l: usize, a: i32, b: i32, expected: &str) {
        let actual = ascend_descend(l, a, b);
        assert!(actual == expected, 
            "With length = {l}, minimum = {a}, maximum = {b}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn example_tests() {
        dotest(5, 1, 3, "12321");
        dotest(14, 0, 2, "01210121012101");
        dotest(11, 5, 9, "56789876567");
    }
}
