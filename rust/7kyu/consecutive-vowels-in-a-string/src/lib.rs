//! https://www.codewars.com/kata/62a933d6d6deb7001093de16/train/rust

fn next_target(c: char) -> char {
    match c {
        'a' => 'e',
        'e' => 'i',
        'i' => 'o',
        'o' => 'u',
        'u' => 'a',
        _ => panic!("unexpected char"),
    }
}

pub fn get_the_vowels(word: &str) -> u32 {
    let mut target = 'a';
    let mut cnt = 0;
    
    word.chars().for_each(|c| {
        if c == target {
            target = next_target(target);
            cnt += 1;
        }
    });
    
    cnt
}

#[cfg(test)]
mod tests {
    use super::get_the_vowels;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: u32) {
        assert_eq!(get_the_vowels(s), expected, "{ERR_MSG} with word = \"{s}\"")   
    }

    #[test]
    fn sample_tests() {
        dotest("agrtertyfikfmroyrntbvsukldkfa", 6);
        dotest("erfaiekjudhyfimngukduo", 4);
        dotest("akfheujfkgiaaaofmmfkdfuaiiie", 7);
    }
}
