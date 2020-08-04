use std::cmp::Ordering;
use std::collections::BTreeMap;

#[allow(dead_code)]
fn count_n_digits_roman_num(digits: usize) -> usize {
    let mut count = 0;

    for i in 1..=3999 {
        let num = RomanNum::convert(i);
        if num.symbol.len() == digits {
            count += 1;
        }
    }

    count
}

#[derive(Debug, Hash, Eq)]
struct RomanNum {
    symbol: String,
    value: usize,
}

impl Ord for RomanNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for RomanNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for RomanNum {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl RomanNum {
    fn new<Symbol: Into<String>>(symbol: Symbol, value: usize) -> Self {
        Self {
            symbol: symbol.into(),
            value,
        }
    }

    fn convert(value: usize) -> Self {
        let mut symbol_digits = Self::new_symbol_digits();

        let mut remain = value;
        symbol_digits.iter_mut().rev().for_each(|(symbol, digit)| {
            *digit = remain / symbol.value;
            remain = remain % symbol.value;
        });

        let symbol = symbol_digits
            .iter()
            .rev()
            .fold(String::new(), |symbol, (num, &digit)| {
                symbol
                    + &std::iter::repeat(num.symbol.to_string())
                        .take(digit)
                        .collect::<String>()
            });

        Self::new(symbol, value)
    }

    fn new_symbol_digits() -> BTreeMap<RomanNum, usize> {
        let mut digits = BTreeMap::new();

        digits.insert(RomanNum::new("M", 1000), 0);
        digits.insert(RomanNum::new("CM", 900), 0);
        digits.insert(RomanNum::new("D", 500), 0);
        digits.insert(RomanNum::new("CD", 400), 0);
        digits.insert(RomanNum::new("C", 100), 0);
        digits.insert(RomanNum::new("XC", 90), 0);
        digits.insert(RomanNum::new("L", 50), 0);
        digits.insert(RomanNum::new("XL", 40), 0);
        digits.insert(RomanNum::new("X", 10), 0);
        digits.insert(RomanNum::new("IX", 9), 0);
        digits.insert(RomanNum::new("V", 5), 0);
        digits.insert(RomanNum::new("IV", 4), 0);
        digits.insert(RomanNum::new("I", 1), 0);

        digits
    }
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
