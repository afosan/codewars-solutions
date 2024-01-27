//! https://www.codewars.com/kata/5245a9138ca049e9a10007b8/train/rust

pub fn count_adjacent_pairs(search_string: &str) -> usize {
    search_string.to_lowercase().split_whitespace().fold((0usize, "", false), |(acc_count, acc_word, acc_accounted), word| {
        if word != acc_word {
            (acc_count, word, false)
        } else {
            if acc_accounted {
                (acc_count, acc_word, acc_accounted)
            } else {
                (acc_count + 1, acc_word, true)
            }
        }
    }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            count_adjacent_pairs(&""),
            0,
            "An empty string should return 0"
        );
        assert_eq!(
            count_adjacent_pairs(&"orange Orange kiwi pineapple apple"),
            1,
            "Case should be ignored"
        );
        assert_eq!(count_adjacent_pairs(&"banana banana banana"), 1, "Once a word has been paired, it is ignored. countAdjacentPairs(\"banana banana banana\")");
        assert_eq!(count_adjacent_pairs(&"banana banana banana terracotta banana terracotta terracotta pie!"), 2, "Once a word has been paired, it is ignored. Grab all pairs. countAdjacentPairs(\"banana banana banana terracotta banana terracotta terracotta pie!\")");
        assert_eq!(
            count_adjacent_pairs(&"pineapple apple"),
            0,
            "A pineapple is not an apple. countAdjacentPairs(\"pineapple apple\")"
        );
    }
}
