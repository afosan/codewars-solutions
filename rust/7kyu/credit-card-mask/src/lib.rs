//! https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust

pub fn maskify(cc: &str) -> String {
    cc.chars().rev().enumerate().map(|(i, c)| if i < 4 { c } else { '#' }).collect::<String>().chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
