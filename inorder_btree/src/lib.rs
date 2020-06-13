#[allow(dead_code)]
fn inorder_btree(node: &Option<Box<Node>>) -> Vec<i32> {
    let mut vals = Vec::new();

    match node {
        Some(node) => {
            vals.append(&mut inorder_btree(&node.left));
            vals.push(node.value);
            vals.append(&mut inorder_btree(&node.right));

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

        let expected = vec![5, 3, 15, 6, 10, 30, 9, 2, 8];
        let actual = inorder_btree(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn empty() {
        let input = None;

        let expected: Vec<i32> = vec![];
        let actual = inorder_btree(&input);

        assert_eq!(expected, actual);
    }
}
