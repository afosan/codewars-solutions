//! https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust

pub fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => unreachable!("unexpected input"),
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(update_light("green"), "yellow");
        assert_eq!(update_light("yellow"), "red");
        assert_eq!(update_light("red"), "green");
    }
}
