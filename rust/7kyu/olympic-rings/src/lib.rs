//! https://www.codewars.com/kata/57d06663eca260fe630001cc/train/rust

pub fn olympic_ring(s: &str) -> String {
    let score = s.chars().map(|c| match c {
        'B' => 2,
        'o' | 'O' | 'd' | 'b' | 'p' | 'P' | 'g' | 'e' | 'A' | 'a' | 'R' | 'D' | 'q' | 'Q' => 1,
        _ => 0,
    }).sum::<u64>() / 2;

    if score <= 1 {
        "Not even a medal!"
    } else if score == 2 {
        "Bronze!"
    } else if score == 3 {
        "Silver!"
    } else {
        "Gold!"
    }.into()
}

#[cfg(test)]
mod tests {
    use super::olympic_ring;

    #[test]
    fn basic() {
        assert_eq!(olympic_ring("wHjMudLwtoPGocnJ"), "Bronze!");
        assert_eq!(olympic_ring("eCEHWEPwwnvzMicyaRjk"), "Bronze!"); 
        assert_eq!(olympic_ring("JKniLfLW"), "Not even a medal!"); 
        assert_eq!(olympic_ring("EWlZlDFsEIBufsalqof"), "Silver!"); 
        assert_eq!(olympic_ring("IMBAWejlGRTDWetPS"), "Gold!");
    }
}
