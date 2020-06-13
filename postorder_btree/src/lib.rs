#[allow(dead_code)]
fn postorder_btree(node: &Option<Box<Node>>) -> Vec<i32> {
    let mut vals = Vec::new();

    match node {
        Some(node) => {
            vals.append(&mut postorder_btree(&node.left));
            vals.append(&mut postorder_btree(&node.right));
            vals.push(node.value);

            vals
        }
        None => vals,
    }
}

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    #[allow(dead_code)]
    fn boxed(value: i32, left: Option<Box<Self>>, right: Option<Box<Self>>) -> Box<Self> {
        Box::new(Self { value, left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = Some(Node::boxed(
            10,
            Some(Node::boxed(
                15,
                Some(Node::boxed(3, Some(Node::boxed(5, None, None)), None)),
                Some(Node::boxed(6, None, None)),
            )),
            Some(Node::boxed(
                30,
                None,
                Some(Node::boxed(
                    2,
                    Some(Node::boxed(9, None, None)),
                    Some(Node::boxed(8, None, None)),
                )),
            )),
        ));

        let expected = vec![5, 3, 6, 15, 9, 8, 2, 30, 10];
        let actual = postorder_btree(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let input = None;

        let expected: Vec<i32> = Vec::new();
        let actual = postorder_btree(&input);

        assert_eq!(expected, actual);
    }
}
