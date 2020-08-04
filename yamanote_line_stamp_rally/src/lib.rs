#[allow(dead_code)]
fn cards(n: usize, a: usize, b: usize) -> usize {
    let s = b - a - 1;
    let t = n - b;

    (1 << s) + (1 << t) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five() {
        let expected = 8;
        let actual = cards(5, 1, 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn twenty_nine() {
        let expected = 36863;
        let actual = cards(29, 1, 17);
        assert_eq!(expected, actual);
    }
}
