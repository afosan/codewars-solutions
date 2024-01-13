//! https://www.codewars.com/kata/5616868c81a0f281e500005c/train/rust

pub fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    let participants = st.split(",").filter(|s| s.len() > 0).collect::<Vec<_>>();

    if participants.len() == 0 {
        return "No participants";
    }
    if n > participants.len() {
        return "Not enough participants";
    }

    let mut all = participants
        .iter()
        .map(|s| (s, s.len() as u32 + s.to_lowercase().chars().map(|c| c as u32 - 'a' as u32 + 1).sum::<u32>()))
        .zip(we.iter())
        .map(|((s, som), w)| (s, som as i32 * w))
        .collect::<Vec<_>>();

    all.sort_by_key(|(s, wn)| (-wn, s.to_string()));

    all[n - 1].0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(rank(st, we, n), exp)
    }

    #[test]
    fn basics_rank() {
        testing("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4, "Benjamin");
        testing("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2, "Matthew");
        testing("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4, "Abigail");
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }
}
