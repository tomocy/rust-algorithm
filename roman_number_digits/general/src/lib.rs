#[allow(dead_code)]
fn count_n_digits_roman_num(digits: usize) -> usize {
    let mut count = 0;

    for i in 0..=3999 {
        let num = convert_to_roman_num(i);
        if num.len() == digits {
            count += 1;
        }
    }

    count
}

fn convert_to_roman_num(value: usize) -> String {
    let (m, remain) = divide(value, 1000);
    let (c, remain) = divide(remain, 100);
    let (x, remain) = divide(remain, 10);

    let mut num = "M".repeat(m);

    num += &convert_one_digit_to_roman_num(c, "M", "D", "C");
    num += &convert_one_digit_to_roman_num(x, "C", "L", "X");
    num += &convert_one_digit_to_roman_num(remain, "X", "V", "I");

    num
}

fn convert_one_digit_to_roman_num(value: usize, x: &str, v: &str, i: &str) -> String {
    if value == 9 {
        i.to_string() + x
    } else if value == 4 {
        i.to_string() + v
    } else {
        let (vs, is) = divide(value, 5);

        v.repeat(vs) + &i.repeat(is)
    }
}

fn divide(n: usize, d: usize) -> (usize, usize) {
    (n / d, n % d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        let expected = 7;
        let actual = count_n_digits_roman_num(1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn twelve() {
        let expected = 93;
        let actual = count_n_digits_roman_num(12);
        assert_eq!(expected, actual);
    }

    #[test]
    fn fifteen() {
        let expected = 1;
        let actual = count_n_digits_roman_num(15);
        assert_eq!(expected, actual);
    }
}
