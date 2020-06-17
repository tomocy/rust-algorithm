use std::collections::hash_map::DefaultHasher;
use std::hash;
use std::hash::Hasher;

#[allow(dead_code)]
const SIZE: usize = 6;

#[allow(dead_code)]
#[derive(Debug)]
struct HashTable<K, V>
where
    K: hash::Hash + PartialEq,
    V: Copy,
{
    elements: [Vec<Element<K, V>>; SIZE],
}

impl<K, V> HashTable<K, V>
where
    K: hash::Hash + PartialEq,
    V: Copy,
{
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            elements: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, key: K, value: V) {
        let element = Element::new(key, value);
        let hashed_key = self.hash_key(&element.key);

        self.elements[hashed_key].push(element);
    }

    #[allow(dead_code)]
    fn get(&self, key: K) -> Option<&V> {
        let hashed_key = self.hash_key(&key);
        let elements = &self.elements[hashed_key];

        elements.iter().find_map(|element| {
            if element.key == key {
                Some(&element.value)
            } else {
                None
            }
        })
    }

    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        hasher.finish() as usize % SIZE
    }
}

#[derive(Debug)]
struct Element<K, V>
where
    K: PartialEq,
{
    key: K,
    value: V,
}

impl<K, V> Element<K, V>
where
    K: PartialEq,
{
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut target = HashTable::new();
        target.insert("a".to_string(), 1);
        target.insert("g".to_string(), 2);
        target.insert("l".to_string(), 3);
        target.insert("p".to_string(), 4);
        target.insert("u".to_string(), 5);
        target.insert("z".to_string(), 6);
        target.insert("あ".to_string(), 1);
        target.insert("い".to_string(), 2);
        target.insert("う".to_string(), 3);
        target.insert("え".to_string(), 4);
        target.insert("お".to_string(), 5);

        assert_eq!(1, *target.get("a".to_string()).unwrap());
        assert_eq!(2, *target.get("g".to_string()).unwrap());
        assert_eq!(3, *target.get("l".to_string()).unwrap());
        assert_eq!(4, *target.get("p".to_string()).unwrap());
        assert_eq!(5, *target.get("u".to_string()).unwrap());
        assert_eq!(6, *target.get("z".to_string()).unwrap());
        assert_eq!(1, *target.get("あ".to_string()).unwrap());
        assert_eq!(2, *target.get("い".to_string()).unwrap());
        assert_eq!(3, *target.get("う".to_string()).unwrap());
        assert_eq!(4, *target.get("え".to_string()).unwrap());
        assert_eq!(5, *target.get("お".to_string()).unwrap());

        println!("{:?}", target);
    }

    #[test]
    fn empty() {
        let target: HashTable<String, i32> = HashTable::new();

        assert_eq!(None, target.get("a".to_string()));
    }
}
