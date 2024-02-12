//! https://www.codewars.com/kata/64fbfe2618692c2018ebbddb/train/rust

pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut switch = true;
    
    list
        .iter()
        .map(|s| {
            if *s == "flick" {
                switch = !switch;
            }
            
            switch
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    
    use super::flick_switch;
    
    fn test_flick<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        assert_eq!(flick_switch(strings), expected);
    }

    #[test]
    fn fixed_tests() {
        test_flick(["codewars", "flick", "code", "wars"], [true, false, false, false]);
        test_flick(["flick", "11037", "3.14", "53"], [false, false, false, false]);
        test_flick(["false", "false", "flick", "sheep", "flick"], [true, true, false, false, true]);
        test_flick(["bicycle"], [true]);
        test_flick(["john, smith, susan", "flick"], [true, false]);
        test_flick(["flick", "flick", "flick", "flick", "flick"], [false, true, false, true, false]);
        test_flick([], []);
    }
}
