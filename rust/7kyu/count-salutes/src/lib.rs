//! https://www.codewars.com/kata/605ae9e1d2be8a0023b494ed/train/rust

pub fn count_salutes(hallway: &str) -> usize {
    let mut right = 0_usize;
    let mut meetings = 0_usize;
    
    hallway.chars().for_each(|c| match c {
        '>' => { right += 1; },
        '<' => { meetings += right; },
        _ => {},
    });
    
    2 * meetings
}

#[cfg(test)]
mod tests {
    use super::count_salutes;

    #[test]
    fn basic() {
        assert_eq!(count_salutes("<---->---<---<-->"), 4);
        assert_eq!(count_salutes("------"), 0);
        assert_eq!(count_salutes(">>>>>>>>>>>>>>>>>>>>>----<->"), 42);
        assert_eq!(count_salutes("<<----<>---<"), 2);
        assert_eq!(count_salutes(">"), 0);
    }  
}
