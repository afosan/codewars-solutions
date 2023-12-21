//! https://www.codewars.com/kata/57ea5b0b75ae11d1e800006c/train/rust

pub fn sort_by_length(arr: &[String]) -> Vec<String> {
    let mut out = arr.to_owned();

    out.sort_by_key(|w| w.len());

    out
}

#[cfg(test)]
mod tests {
    use super::sort_by_length;
        
    fn dotest(arr: &[String], expected: &[String]) {
        let actual = sort_by_length(arr);
        assert!(actual == expected, 
            "With arr = {arr:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[String::from("beg"), String::from("life"), String::from("i"), String::from("to")], &[String::from("i"), String::from("to"), String::from("beg"), String::from("life")]);
        dotest(&[String::from(""), String::from("moderately"), String::from("brains"), String::from("pizza")], &[String::from(""), String::from("pizza"), String::from("brains"), String::from("moderately")]);
        dotest(&[String::from("longer"), String::from("longest"), String::from("short")], &[String::from("short"), String::from("longer"), String::from("longest")]);
        dotest(&[String::from("dog"), String::from("food"), String::from("a"), String::from("of")], &[String::from("a"), String::from("of"), String::from("dog"), String::from("food")]);
        dotest(&[String::from(""), String::from("dictionary"), String::from("eloquent"), String::from("bees")], &[String::from(""), String::from("bees"), String::from("eloquent"), String::from("dictionary")]);
        dotest(&[String::from("a longer sentence"), String::from("the longest sentence"), String::from("a short sentence")], &[String::from("a short sentence"), String::from("a longer sentence"), String::from("the longest sentence")]);
    }
}
