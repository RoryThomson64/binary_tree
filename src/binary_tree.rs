impl<T: std::cmp::PartialOrd> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    pub fn add(&mut self, value: T) {
        // if value less than stored val
        //Go left. If leaf, add node

        //else go right

        if self.value > value {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(value)));
            } else {
                self.left.as_mut().unwrap().add(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(value)));
            } else {
                self.right.as_mut().unwrap().add(value);
            }
        }
    }

    pub fn search(&self, search_val: T) -> bool {
        if (search_val == self.value) {
            return true;
        }
        if (search_val > self.value) {
            if self.right.is_none() {
                return false;
            }
            return self.right.as_ref().unwrap().search(search_val);
        }
        if (search_val < self.value) {
            if self.left.is_none() {
                return false;
            }
            return self.left.as_ref().unwrap().search(search_val);
        }
        return false;
    }
}

#[derive(Debug)]
pub struct Node<T: std::cmp::PartialOrd> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
