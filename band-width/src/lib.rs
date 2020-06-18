use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    occupation: Occupation,
    neighors: Vec<Box<Person>>,
}

impl Person {
    #[allow(dead_code)]
    fn new(name: &str, occupation: Occupation, neighors: Vec<&Person>) -> Self {
        Self {
            name: name.to_string(),
            occupation,
            neighors: neighors
                .iter()
                .map(|&person| Box::new(person.clone()))
                .collect(),
        }
    }

    #[allow(dead_code)]
    fn find_professional(&self, occupation: Occupation) -> Option<&Person> {
        let mut queue = Queue::new();
        queue.push(self);
        queue.find_professional(occupation)
    }
}

#[derive(Debug)]
struct Queue<'a> {
    queue: Vec<&'a Person>,
    queued_people: HashMap<String, bool>,
}

impl<'a> Queue<'a> {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
            queued_people: HashMap::new(),
        }
    }

    fn find_professional(&mut self, occupation: Occupation) -> Option<&'a Person> {
        loop {
            match self.pop() {
                Some(person) => {
                    if person.occupation == occupation {
                        return Some(person);
                    } else {
                        person.neighors.iter().for_each(|person| self.push(person));
                        continue;
                    }
                }
                None => return None,
            }
        }
    }

    fn push(&mut self, person: &'a Person) {
        if self.queued_people.contains_key(&person.name) {
            return;
        }

        self.queue.push(person);
    }

    fn pop(&mut self) -> Option<&'a Person> {
        if self.queue.len() == 0 {
            None
        } else {
            let first = self.queue[0];
            self.queue = self.queue[1..].iter().map(|&person| person).collect();

            Some(first)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Occupation {
    None,
    AppleSeller,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighor_is_professional() {
        let anuj = Person::new("anuj", Occupation::None, vec![]);
        let thom = Person::new("thom", Occupation::None, vec![]);
        let jonny = Person::new("jonny", Occupation::None, vec![]);
        let peggy = Person::new("peggy", Occupation::AppleSeller, vec![]);
        let alice = Person::new("alice", Occupation::None, vec![&peggy]);
        let bob = Person::new("bob", Occupation::None, vec![&anuj, &peggy]);
        let clare = Person::new("clare", Occupation::None, vec![&jonny, &thom]);

        let you = Person::new("you", Occupation::None, vec![&alice, &bob, &clare]);

        assert_eq!(
            "peggy",
            you.find_professional(Occupation::AppleSeller).unwrap().name
        );
    }

    #[test]
    fn nobody_is_professional() {
        let anuj = Person::new("anuj", Occupation::None, vec![]);
        let thom = Person::new("thom", Occupation::None, vec![]);
        let jonny = Person::new("jonny", Occupation::None, vec![]);
        let peggy = Person::new("peggy", Occupation::None, vec![]);
        let alice = Person::new("alice", Occupation::None, vec![&peggy]);
        let bob = Person::new("bob", Occupation::None, vec![&anuj, &peggy]);
        let clare = Person::new("clare", Occupation::None, vec![&jonny, &thom]);

        let you = Person::new("you", Occupation::None, vec![&alice, &bob, &clare]);

        assert_eq!(
            true,
            you.find_professional(Occupation::AppleSeller).is_none()
        );
    }

    #[test]
    fn you_are_professional() {
        let anuj = Person::new("anuj", Occupation::None, vec![]);
        let thom = Person::new("thom", Occupation::None, vec![]);
        let jonny = Person::new("jonny", Occupation::None, vec![]);
        let peggy = Person::new("peggy", Occupation::None, vec![]);
        let alice = Person::new("alice", Occupation::None, vec![&peggy]);
        let bob = Person::new("bob", Occupation::None, vec![&anuj, &peggy]);
        let clare = Person::new("clare", Occupation::None, vec![&jonny, &thom]);

        let you = Person::new("you", Occupation::AppleSeller, vec![&alice, &bob, &clare]);

        assert_eq!(
            "you",
            you.find_professional(Occupation::AppleSeller).unwrap().name
        );
    }

    #[test]
    fn you_know_no_neighors() {
        let you = Person::new("you", Occupation::None, vec![]);

        assert_eq!(
            true,
            you.find_professional(Occupation::AppleSeller).is_none()
        );
    }

    #[test]
    fn you_and_neighor_know_each_other() {
        let mut peggy = Person::new("you", Occupation::None, vec![]);
        let you = Person::new("you", Occupation::None, vec![&peggy]);
        peggy.neighors.push(Box::new(you.clone()));

        assert_eq!(
            true,
            you.find_professional(Occupation::AppleSeller).is_none()
        );
    }
}
