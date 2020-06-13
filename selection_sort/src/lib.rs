#[allow(dead_code)]
fn selection_sort(list: &Vec<i32>) -> Vec<i32> {
    let mut list = list.clone();

    for i in 0..list.len() {
        if let Some((min, _)) = list.iter().enumerate().skip(i).min_by_key(|x| x.1) {
            list.swap(i, min);
        }
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_in_empty() {
        let expected: Vec<i32> = Vec::new();
        let actual = selection_sort(&vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn success() {
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let actual = selection_sort(&vec![0, 7, 2, 5, 1, 6, 4, 9, 3, 8]);
        assert_eq!(expected, actual);
    }
}
