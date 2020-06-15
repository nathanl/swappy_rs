#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(Vec<usize>);

impl Node {
    pub fn new(vec: Vec<usize>) -> Node {
        Node(vec)
    }

    pub fn parent(&self) -> Node {
        let mut parent_vec = self.0.clone();
        parent_vec.pop();
        Node(parent_vec)
    }

    pub fn first_child(&self) -> Node {
        let mut child_vec = self.0.clone();
        let new_level = match child_vec.last() {
            Some(i) => i.clone(),
            None => 0 as usize
        };
        child_vec.push(new_level);
        Node(child_vec)
    }

    pub fn next(&self, max_index : usize) -> Node {
        match self.0.last() {
            None => self.clone(),
            Some(i) => {
                let next_index = i + 1;
                if next_index > max_index {
                    self.parent().next(max_index)
                } else {
                    let mut next_vec = self.0.clone();
                    next_vec.pop();
                    next_vec.push(next_index);
                    Node(next_vec)
                }
            }
        }
    }

    fn clone(&self) -> Node {
        Node(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_parent() {
        let one_two = Node::new(vec![1,2]);
        let expected_parent = Node::new(vec![1]);
        assert_eq!(one_two.parent(), expected_parent); 
    }

    #[test]
    fn gets_first_child() {
        let two_five = Node::new(vec![2,5]);
        let expected_child = Node::new(vec![2,5,5]);
        assert_eq!(two_five.first_child(), expected_child);
    }

    #[test]
    fn gets_next() {
        let max_index = 10 as usize;

        let one_four = Node::new(vec![1,4]);
        let expected_sibling = Node::new(vec![1,5]);
        assert_eq!(one_four.next(max_index), expected_sibling);

        let one_ten = Node::new(vec![1,10]);
        let expected_uncle = Node::new(vec![2]);
        assert_eq!(one_ten.next(max_index), expected_uncle);
    }
}
