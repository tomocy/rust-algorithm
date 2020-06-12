#[allow(dead_code)]
fn binary_search(set: &Vec<i32>, target: i32) -> Option<usize> {
    let mut low: i32 = 0;
    let mut high: i32 = set.len() as i32;

    while low <= high {
        let middle = (low + high) / 2;
        let guess = set[middle as usize];

        if guess == target {
            return Some(middle as usize);
        }

        if guess <= target {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_something() {
        let set = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(5, binary_search(&set, 5).unwrap());
    }

    #[test]
    fn find_nothing() {
        let set = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(true, binary_search(&set, -1).is_none());
    }
}
