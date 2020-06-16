#[allow(dead_code)]
fn quick_sort(vals: &[i32]) -> Vec<i32> {
    match vals.len() {
        0 => Vec::new(),
        1 => vec![vals[0]],
        _ => {
            let pivot = vals[0];

            let less: Vec<i32> = vals
                .iter()
                .skip(1)
                .filter(|&&val| val <= pivot)
                .map(|&val| val)
                .collect();
            let greater: Vec<i32> = vals
                .iter()
                .skip(1)
                .filter(|&&val| val > pivot)
                .map(|&val| val)
                .collect();

            let mut sorted = quick_sort(&less);
            sorted.append(&mut vec![pivot]);
            sorted.append(&mut quick_sort(&greater));

            sorted
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![6, 2, 4];

        let expected = vec![2, 4, 6];
        let actual = quick_sort(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let input = Vec::new();

        let expected: Vec<i32> = Vec::new();
        let actual = quick_sort(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn single() {
        let input = vec![2];

        let expected = vec![2];
        let actual = quick_sort(&input);

        assert_eq!(expected, actual);
    }
}
