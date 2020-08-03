#[allow(dead_code)]
fn n_c_r(n: usize, r: usize) -> usize {
    factorial(n) / (factorial(n - r) * factorial(r))
}

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_c_zero() {
        let expected = 1;
        let actual = n_c_r(5, 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five_c_one() {
        let expected = 5;
        let actual = n_c_r(5, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five_c_two() {
        let expected = 10;
        let actual = n_c_r(5, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five_c_three() {
        let expected = 10;
        let actual = n_c_r(5, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five_c_four() {
        let expected = 5;
        let actual = n_c_r(5, 4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn five_c_five() {
        let expected = 1;
        let actual = n_c_r(5, 5);
        assert_eq!(expected, actual);
    }
}
