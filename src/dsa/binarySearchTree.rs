#[derive(Clone)]

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(value: i32) -> Self {
        Node {
            val: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value > self.val {
            match self.right {
                None => {
                    self.right = Some(Box::new(Node {
                        val: value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        } else if value< self.val {
            match self.left {
                None => {
                    self.left = Some(Box::new(Node {
                        val: value,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(value),
            }
        }else{
            return;
        }
    }
}

pub struct BinarySearchTree {
    root: Node,
}

impl BinarySearchTree {
    pub fn new(value: i32) -> Self {
        BinarySearchTree {
            root: Node::new(value),
        }
    }
    fn traversal(node: &Option<Box<Node>>, high: i32, low: i32,  output: &mut Vec<i32>) {
        if let Some(node) = node {
            if node.val >= low && node.val <= high {
                // println!("adding this: {} ", node.val);
                output.push(node.val);
            }
            if node.val >= low {
                // println!("going left");
                Self::traversal(&node.left, high, low, output);
            }
            if node.val <= high {
                // println!("going right");
                Self::traversal(&node.right, high, low, output);
            }
        }
        else {
            println!("node is none");
        }
        println!("final output: {:?} ", output);
    }
    pub fn insert(&mut self, value: i32) {
        self.root.insert(value);
    }

    pub fn productsInRange(&self, low: i32, high: i32) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        println!("min: {}, max: {}", low, high);
        Self::traversal(&Some(Box::new(self.root.clone())), high, low, &mut output);
        output
    }
}

