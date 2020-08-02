use std::cmp;

#[allow(dead_code)]
fn count_groups(remain: usize, max_per_table: usize, prev_groups: Option<usize>) -> usize {
    if prev_groups.is_some() && remain == 0 {
        1
    } else {
        let mut groups = 0;
        let prev_groups = match prev_groups {
            Some(prev_groups) => prev_groups,
            None => 2,
        };
        let max_per_table = cmp::min(max_per_table, remain);

        for i in prev_groups..=max_per_table {
            groups += count_groups(remain - i, max_per_table, Some(i));
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let expected = 0;
        let actual = count_groups(0, 0, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one() {
        let expected = 0;
        let actual = count_groups(1, 1, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn two() {
        let expected = 1;
        let actual = count_groups(2, 2, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn three() {
        let expected = 1;
        let actual = count_groups(3, 3, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn four() {
        let expected = 2;
        let actual = count_groups(4, 4, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five() {
        let expected = 2;
        let actual = count_groups(5, 5, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn six() {
        let expected = 4;
        let actual = count_groups(6, 6, None);
        assert_eq!(expected, actual);
    }

    #[test]
    fn big() {
        let expected = 437420;
        let actual = count_groups(100, 10, None);
        assert_eq!(expected, actual);
    }
}
