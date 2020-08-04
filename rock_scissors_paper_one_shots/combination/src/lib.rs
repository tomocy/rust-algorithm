#[allow(dead_code)]
fn one_shots(n: usize) -> usize {
    let mut shots = 0;

    for left in 0..=n {
        for right in left..=n {
            let rocks = left;
            let scissors = right - left;
            let papers = n - scissors - rocks;
            let combination = [rocks, scissors, papers];

            if combination
                .iter()
                .filter(|&x| x == combination.iter().max().unwrap())
                .count()
                == 1
            {
                shots += 1;
            }
        }
    }

    shots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four() {
        let expected = 12;
        let actual = one_shots(4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_hundred() {
        let expected = 5100;
        let actual = one_shots(100);
        assert_eq!(expected, actual);
    }
}
