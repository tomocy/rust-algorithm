use std::collections::{BTreeMap, HashSet};

#[allow(dead_code)]
fn greedy(stations: &BTreeMap<&str, HashSet<&str>>, needed_states: &HashSet<&str>) -> Vec<String> {
    let mut best_covering_stations = Vec::new();
    let mut needed_states = needed_states.clone();

    while !needed_states.is_empty() {
        let mut best_covering_station = String::new();
        let mut best_covered_states = HashSet::new();

        stations.iter().for_each(|(&station, states)| {
            let covered_states: HashSet<_> = needed_states
                .intersection(states)
                .map(|state| *state)
                .collect();
            if covered_states.len() > best_covered_states.len() {
                best_covering_station = station.to_string();
                best_covered_states = covered_states;
            }
        });

        best_covering_stations.push(best_covering_station);
        needed_states = needed_states
            .difference(&best_covered_states)
            .map(|state| *state)
            .collect();
    }

    best_covering_stations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn basic() {
        let mut states = BTreeMap::new();
        states.insert("kone", HashSet::from_iter(vec!["id", "nv", "ut"]));
        states.insert("ktwo", HashSet::from_iter(vec!["wa", "id", "mt"]));
        states.insert("kthree", HashSet::from_iter(vec!["or", "nv", "ca"]));
        states.insert("kfour", HashSet::from_iter(vec!["nv", "ut"]));
        states.insert("kfive", HashSet::from_iter(vec!["ca", "az"]));

        let needed_states =
            HashSet::from_iter(vec!["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]);

        let mut expected = vec!["kone", "ktwo", "kthree", "kfive"];
        expected.sort();

        let mut actual = greedy(&states, &needed_states);
        actual.sort();

        assert_eq!(expected, actual);
    }
}
