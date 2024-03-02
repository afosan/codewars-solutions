//! https://www.codewars.com/kata/515e188a311df01cba000003/train/rust

pub fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury",
        2 => "Venus",
        3 => "Earth",
        4 => "Mars",
        5 => "Jupiter",
        6 => "Saturn",
        7 => "Uranus",
        8 => "Neptune",
        _ => unreachable!(),
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_planet_name() {
        assert_eq!(get_planet_name(3), "Earth");
    }
}
