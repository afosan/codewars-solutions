//! https://www.codewars.com/kata/581331293788bc1702001fa6/train/rust

pub fn mirror(text: &str) -> String {
    let words = text.split_whitespace().collect::<Vec<_>>();
    let l = words.iter().map(|w| w.len()).max().unwrap_or(0) + 4;
    let star_line = "*".to_string().repeat(l);
    
    let mut lines = vec![star_line.clone()];
    lines.extend(words.into_iter().map(|w| {
        let word = w.chars().rev().collect::<String>();
        format!("* {}{}*", word, " ".to_string().repeat(l - 3 - word.len()))
    }).collect::<Vec<_>>());
    lines.push(star_line);
    
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::mirror;
        
    fn dotest(s: &str, expected: &str) {
        let actual = mirror(s);
        assert!(actual == expected, 
            "With text = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("Hello World", "*********\n* olleH *\n* dlroW *\n*********");
        dotest("Codewars", "************\n* srawedoC *\n************"); 
        dotest("emosewA !ataK", "***********\n* Awesome *\n* Kata!   *\n***********");
    }
}
