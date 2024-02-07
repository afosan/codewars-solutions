//! https://www.codewars.com/kata/5bbd279c8f8bbd5ee500000f/train/rust

pub fn find_screen_height(width: u64, ratio: &str) -> String {
    let mut iter = ratio.split(":");
    let n1 = iter.next().expect("expect at least one element").parse::<u64>().expect("cannot parse into u64");
    let n2 = iter.next().expect("expect at least two elements").parse::<u64>().expect("cannot parse into u64");
    
    format!("{}x{}", width, (width as f64 / n1 as f64 * n2 as f64).floor() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // 4:3, width = 768
        assert_eq!(find_screen_height(1024,"4:3"), "1024x768");
        
        // 16:9, width = 720
        assert_eq!(find_screen_height(1280,"16:9"), "1280x720");
        
        // 32:9, width = 1080
        assert_eq!(find_screen_height(3840,"32:9"), "3840x1080");   
    }
}
