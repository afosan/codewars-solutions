//! https://www.codewars.com/kata/57e90bcc97a0592126000064/train/rust

pub fn sea_sick(sea: &str) -> &'static str {
    let changes = sea.chars().collect::<Vec<_>>().windows(2).filter(|ss| ss[0] != ss[1]).count();
    let length = sea.len();
    
    if changes as f64 / length as f64 > 0.2 {
        "Throw Up"
    } else {
        "No Problem"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(sea_sick("~"), "No Problem");
        assert_eq!(sea_sick("_~~~~~~~_~__~______~~__~~_~~"), "Throw Up");
        assert_eq!(sea_sick("______~___~_"), "Throw Up");
        assert_eq!(sea_sick("____"), "No Problem");
        assert_eq!(sea_sick("_~~_~____~~~~~~~__~_~"), "Throw Up");
        assert_eq!(sea_sick( "~~~~~_~~~~"), "No Problem");
    }
}
