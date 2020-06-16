#[allow(dead_code)]
fn max(vals: &[i32]) -> Option<i32> {
    match vals.len() {
        0 => None,
        1 => Some(vals[0]),
        _ => {
            let a = vals[0];
            let b = max(&vals[1..]).unwrap();

            if a > b {
                Some(a)
            } else {
                Some(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = vec![2, 6, 4];

        let expected = 6;
        let actual = max(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let input = Vec::new();

        let expected = None;
        let actual = max(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn single() {
        let input = vec![2];

        let expected = 2;
        let actual = max(&input).unwrap();

        assert_eq!(expected, actual);
    }
}
