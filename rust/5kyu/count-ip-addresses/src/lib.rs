//! https://www.codewars.com/kata/526989a41034285187000de4/train/rust

fn ip_to_number(ip: &str) -> u32 {
    ip
        .split(".")
        .map(|w| w.parse::<u32>().expect("not a valid number string"))
        .fold(0_u32, |mut acc, n| {acc = 256 * acc + n; acc})
}

pub fn ips_between(start: &str, end: &str) -> u32 {
    ip_to_number(end) - ip_to_number(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
