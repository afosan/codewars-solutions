#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    if arr.len() == 0 {
        return vec![];
    }

    let mut inc = vec![true; arr.len()];
    let mut i = 0;
    let mut is_reduc = false;

    use Direction as D;
    while i < arr.len() - 1 {
        match (arr[i], arr[i + 1]) {
            (D::North, D::South) | (D::South, D::North) | (D::West, D::East) | (D::East, D::West) => {
                inc[i] = false;
                inc[i + 1] = false;
                i += 2;
                is_reduc = true;
            },
            _ => {
                i += 1;
            }
        }
    }

    if is_reduc {
        dir_reduc(
            &arr.iter().zip(inc.iter()).filter(|t| *t.1).map(|t| t.0).cloned().collect::<Vec<_>>()[..]
        )
    } else {
        arr.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::{self, *}};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);
        
        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);

        let a = [];
        assert_eq!(dir_reduc(&a), []);
    }
}
