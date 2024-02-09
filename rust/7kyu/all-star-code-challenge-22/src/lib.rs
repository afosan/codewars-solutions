//! https://www.codewars.com/kata/5865cff66b5699883f0001aa/train/rust

pub fn to_time(seconds: u32) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;

    format!("{h} hour(s) and {m} minute(s)")
}

#[cfg(test)]
mod tests {
    use super::to_time;

    #[test]
    fn basic() {
        assert_eq!(to_time(3_600), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_601), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_500), "0 hour(s) and 58 minute(s)");
        assert_eq!(to_time(323_500), "89 hour(s) and 51 minute(s)");
        assert_eq!(to_time(0), "0 hour(s) and 0 minute(s)");
    }
}
