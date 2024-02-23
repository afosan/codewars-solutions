//! https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust

pub fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut cubes = cubes.to_vec();
    cubes.sort_unstable();

    if dir == 'L' {
        cubes = cubes.into_iter().rev().collect();
    }
    
    cubes
}

#[cfg(test)]
mod tests {
    use super::flip;

    #[test]
    fn sample_tests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
