#[allow(dead_code)]
fn sum(vals: &[i32]) -> i32 {
    match vals.len() {
        0 => 0,
        1 => vals[0],
        _ => vals[0] + sum(&vals[1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![2, 4, 6];

        let expected = 12;
        let actual = sum(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let input: Vec<i32> = Vec::new();

        let expected = 0;
        let actual = sum(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn single() {
        let input = vec![2];

        let expected = 2;
        let actual = sum(&input);

        assert_eq!(expected, actual);
    }
}
