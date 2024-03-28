//! https://www.codewars.com/kata/62eb800ba29959001c07dfee/train/rust

fn brightness(s: &String) -> u64 {
    let s = s.to_lowercase();
    let r = u64::from_str_radix(&s[1..3], 16).unwrap();
    let g = u64::from_str_radix(&s[3..5], 16).unwrap();
    let b = u64::from_str_radix(&s[5..7], 16).unwrap();
    
    r.max(g).max(b)
}

pub fn brightest(colors: &[String]) -> String {
    let mut cs = colors.iter().enumerate().collect::<Vec<_>>();
    
    cs.sort_unstable_by_key(|(i, c)| (brightness(c), -(*i as i64)));
    
    cs.iter().last().unwrap().1.to_owned()
}

#[cfg(test)]
mod tests {
    use super::brightest;
        
    fn dotest(colors: &[String], expected: &str) {
        let actual = brightest(colors);
        assert!(actual == expected, "With colors = {colors:?}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[String::from("#001000"), String::from("#000000")], "#001000");
        dotest(&[String::from("#ABCDEF"), String::from("#123456")], "#ABCDEF");
        dotest(&[String::from("#00FF00"), String::from("#FFFF00")], "#00FF00");
        dotest(&[String::from("#FFFFFF"), String::from("#1234FF")], "#FFFFFF");
        dotest(&[String::from("#FFFFFF"), String::from("#123456"), String::from("#000000")], "#FFFFFF");
    }
}
