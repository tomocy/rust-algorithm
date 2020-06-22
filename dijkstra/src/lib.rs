use std::collections::HashMap;

#[allow(dead_code)]
struct Node<'a> {
    name: String,
    edges: Vec<Edge<'a>>,
}

impl<'a> Node<'a> {
    #[allow(dead_code)]
    fn new(name: &str, edges: Vec<Edge<'a>>) -> Self {
        Self {
            name: name.to_string(),
            edges,
        }
    }

    #[allow(dead_code)]
    fn lowest_cost(&self, dst: &str) -> Option<u32> {
        let mut table = CostTable::new(self);

        while let Some((name, cost)) = table.lowest_cost() {
            self.neighor(&name)
                .unwrap()
                .edges
                .iter()
                .for_each(|edge| table.save_lower(&edge.node.name, cost + edge.weight));
        }

        table.cost(dst)
    }

    fn neighor(&self, name: &str) -> Option<&Node> {
        self.edges.iter().find_map(|edge| {
            if edge.node.name == name {
                Some(edge.node)
            } else {
                edge.node.neighor(name)
            }
        })
    }
}

struct Edge<'a> {
    weight: u32,
    node: &'a Node<'a>,
}

impl<'a> Edge<'a> {
    #[allow(dead_code)]
    fn new(weight: u32, node: &'a Node<'a>) -> Self {
        Self { weight, node }
    }
}

struct CostTable {
    costs: HashMap<String, u32>,
    proceeded: HashMap<String, bool>,
}

impl CostTable {
    fn new(node: &Node) -> Self {
        let mut table = Self {
            costs: HashMap::new(),
            proceeded: HashMap::new(),
        };

        node.edges
            .iter()
            .for_each(|edge| table.save_lower(&edge.node.name, edge.weight));

        table
    }

    fn save_lower(&mut self, name: &str, cost: u32) {
        match self.costs.get(name) {
            Some(&stored) if stored <= cost => {}
            _ => {
                self.costs.insert(name.to_string(), cost);
            }
        }
    }

    fn lowest_cost(&mut self) -> Option<(String, u32)> {
        match self
            .costs
            .iter()
            .filter(|(name, _)| !self.proceeded.contains_key(*name))
            .min_by(|(_, a), (_, b)| a.cmp(b))
        {
            Some((name, cost)) => {
                self.proceeded.insert(name.clone(), true);
                Some((name.clone(), *cost))
            }
            None => None,
        }
    }

    fn cost(&self, name: &str) -> Option<u32> {
        match self.costs.get(name) {
            Some(&cost) => Some(cost),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let finish = Node::new("finish", vec![]);
        let a = Node::new("a", vec![Edge::new(1, &finish)]);
        let b = Node::new("b", vec![Edge::new(3, &a), Edge::new(5, &finish)]);
        let start = Node::new("start", vec![Edge::new(6, &a), Edge::new(2, &b)]);

        let expected = 6;
        let actual = start.lowest_cost("finish").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn not_found() {
        let finish = Node::new("finish", vec![]);
        let a = Node::new("a", vec![Edge::new(1, &finish)]);
        let b = Node::new("b", vec![Edge::new(3, &a), Edge::new(5, &finish)]);
        let start = Node::new("start", vec![Edge::new(6, &a), Edge::new(2, &b)]);

        assert_eq!(true, start.lowest_cost("end").is_none());
    }
}
