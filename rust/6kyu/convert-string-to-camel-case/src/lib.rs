//! https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust

pub fn to_camel_case(text: &str) -> String {
    text.chars().fold(
        (
            Vec::<char>::with_capacity(text.len()),
            false
        ),
        |(mut out, mut flag), c| {
            match c {
                '-' | '_' => { flag = true; },
                _ => {
                    if flag {
                        out.push(c.to_ascii_uppercase());
                        flag = false;
                    } else {
                        out.push(c);
                    }
                }
            };
            
            (out, flag)
        }
    ).0.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("","");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}
