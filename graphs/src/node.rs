pub struct Node<T> {
    value: T,
    children: Vec<*const Node<T>>,
}

impl<T: Default> Node<T> {
    fn new() -> Self {
        Node {
            value: T::default(),
            children: Vec::new(),
        }
    }
}

impl<T> From<T> for Node<T> {
    fn from(val: T) -> Self {
        Node {
            value: val,
            children: Vec::new(),
        }
    }
}

impl<T: Copy> Node<T> {
    fn get_value(&self) -> T {
        self.value
    }
}

impl<T> Node<T> {
    fn ref_value(&self) -> &T {
        &self.value
    }

    fn mut_ref_value(&mut self) -> &mut T {
        &mut self.value
    }

    fn add_child(&mut self, child: &Node<T>) {
        self.children.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node: Node<i32> = Node::new();
        assert_eq!(node.value, 0);
        assert!(node.children.is_empty());

        let node: Node<f64> = Node::new();
        assert_eq!(node.value, 0.);
        assert!(node.children.is_empty());

        let node: Node<&str> = Node::new();
        assert_eq!(node.value, "");
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_from() {
        let node = Node::from(5);
        assert_eq!(node.value, 5);
        assert!(node.children.is_empty());

        let node = Node::from(3.14);
        assert_eq!(node.value, 3.14);
        assert!(node.children.is_empty());

        let node = Node::from("pizza");
        assert_eq!(node.value, "pizza");
        assert!(node.children.is_empty());
    }
}
