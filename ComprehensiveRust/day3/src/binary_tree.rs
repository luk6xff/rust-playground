//------------------------------ 1st Implementation ------------------------------
enum BinaryTree<T: Ord> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

struct Node<T: Ord> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree::Empty
    }

    fn insert(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(Node {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => match value.cmp(&node.element) {
                std::cmp::Ordering::Less => node.left.insert(value),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => node.right.insert(value),
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            BinaryTree::Empty => 0,
            BinaryTree::NonEmpty(node) => 1 + node.left.len() + node.right.len(),
        }
    }

    fn has(&self, value: &T) -> bool {
        match self {
            BinaryTree::Empty => false,
            BinaryTree::NonEmpty(node) => match value.cmp(&node.element) {
                std::cmp::Ordering::Less => node.left.has(value),
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => node.right.has(value),
            }
        }
    }

}




//------------------------------ 2nd Alternative implementation ------------------------------
#[derive(Debug)]
pub struct BinaryTree2<T: Ord> {
    root: Subtree<T>,
}

/// A node in the binary tree.
#[derive(Debug)]
struct Node2<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node2<T>>>);


// Implement `new`, `insert`, `len`, and `has`.
impl<T: Ord> BinaryTree2<T> {
    fn new() -> Self {
        BinaryTree2 { root: Subtree(None) }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }
}

impl<T: Ord> Subtree<T> {
    fn insert(&mut self, value: T) {
        match & mut self.0 {
            None => self.0 = Some(Box::new(Node2 {
                value,
                left: Subtree(None),
                right: Subtree(None),
            })),
            Some(n) => match value.cmp(&n.value) {
                std::cmp::Ordering::Less => n.left.insert(value),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => n.right.insert(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                std::cmp::Ordering::Less => n.left.has(value),
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => n.right.has(value),
            }
        }
    }
}






// 3rd  Alternative implementation
#[derive(Debug)]
pub struct BinaryTree3<T: Ord> {
    value: Option<T>,
    left: Option<Box<BinaryTree3<T>>>,
    right: Option<Box<BinaryTree3<T>>>,
}

impl<T> Default for BinaryTree3<T>
where
    T: Ord {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: Ord> BinaryTree3<T> {
    fn new() -> Self {
        BinaryTree3 {
            value: None,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(ref v) => match v.cmp(&value) {
                std::cmp::Ordering::Less => {
                    if let Some(left) = &mut self.left {
                        left.insert(value);
                    } else {
                        self.left = Some(Box::new(BinaryTree3 {
                            value: Some(value),
                            left: None,
                            right: None
                        }));
                    }
                }
                std::cmp::Ordering::Greater => {
                    if let Some(right) = &mut self.right {
                        right.insert(value);
                    } else {
                        self.right = Some(Box::new(BinaryTree3 {
                            value: Some(value),
                            left: None,
                            right: None
                        }));
                    }
                }
                std::cmp::Ordering::Equal => (),
            }
        }
    }

    fn len(&self) -> usize {
        match &self.value {
            None => 0,
            // Some(_) => {
                // let left_len = self.left.as_ref().map_or(0, |tree| tree.len());
                // let right_len = self.right.as_ref().map_or(0, |tree| tree.len());
                // 1 + left_len + right_len
            // }
            Some(_) => {
                let mut count = 0;
                if self.value.is_some() {
                    count += 1;
                }
                if let Some(left) = &self.left {
                    count += left.len();
                }
                if let Some(right) = &self.right {
                    count += right.len();
                }
                count
            }
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.value {
            None => false,
            Some(v) => match v.cmp(&value) {
                std::cmp::Ordering::Less => self.left.as_ref().map_or(false, |tree| tree.has(value)),
                std::cmp::Ordering::Greater => self.right.as_ref().map_or(false, |tree| tree.has(value)),
                std::cmp::Ordering::Equal => true,
            },
        }
    }
}




#[test]
fn len() {

    // BinaryTree
    let mut tree = BinaryTree::new();
    assert_eq!(tree.len(), 0);
    tree.insert(2);
    assert_eq!(tree.len(), 1);
    tree.insert(1);
    assert_eq!(tree.len(), 2);
    tree.insert(2); // not a unique item
    assert_eq!(tree.len(), 2);

    // BinaryTree2
    let mut tree = BinaryTree2::new();
    assert_eq!(tree.len(), 0);
    tree.insert(2);
    assert_eq!(tree.len(), 1);
    tree.insert(1);
    assert_eq!(tree.len(), 2);
    tree.insert(2); // not a unique item
    assert_eq!(tree.len(), 2);

    // BinaryTree3
    let mut tree = BinaryTree3::new();
    assert_eq!(tree.len(), 0);
    tree.insert(2);
    assert_eq!(tree.len(), 1);
    tree.insert(1);
    assert_eq!(tree.len(), 2);
    tree.insert(2); // not a unique item
    assert_eq!(tree.len(), 2);
}

#[test]
fn has() {
    // BinaryTree
    let mut tree = BinaryTree::new();
    fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
        let got: Vec<bool> =
            (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
        assert_eq!(&got, exp);
    }
    check_has(&tree, &[false, false, false, false, false]);
    tree.insert(0);
    check_has(&tree, &[true, false, false, false, false]);
    tree.insert(4);
    check_has(&tree, &[true, false, false, false, true]);
    tree.insert(4);
    check_has(&tree, &[true, false, false, false, true]);
    tree.insert(3);
    check_has(&tree, &[true, false, false, true, true]);


    // BinaryTree2
    let mut tree = BinaryTree2::new();
    fn check_has2(tree: &BinaryTree2<i32>, exp: &[bool]) {
        let got: Vec<bool> =
            (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
        assert_eq!(&got, exp);
    }
    check_has2(&tree, &[false, false, false, false, false]);
    tree.insert(0);
    check_has2(&tree, &[true, false, false, false, false]);
    tree.insert(4);
    check_has2(&tree, &[true, false, false, false, true]);
    tree.insert(4);
    check_has2(&tree, &[true, false, false, false, true]);
    tree.insert(3);
    check_has2(&tree, &[true, false, false, true, true]);

    // BinaryTree3
    let mut tree = BinaryTree3::new();
    fn check_has(tree: &BinaryTree3<i32>, exp: &[bool]) {
        let got: Vec<bool> =
            (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
        assert_eq!(&got, exp);
    }
    check_has(&tree, &[false, false, false, false, false]);
    tree.insert(0);
    check_has(&tree, &[true, false, false, false, false]);
    tree.insert(4);
    check_has(&tree, &[true, false, false, false, true]);
    tree.insert(4);
    check_has(&tree, &[true, false, false, false, true]);
    tree.insert(3);
    check_has(&tree, &[true, false, false, true, true]);
}

#[test]
fn unbalanced() {
    // BinaryTree
    let mut tree = BinaryTree::new();
    for i in 0..100 {
        tree.insert(i);
    }
    assert_eq!(tree.len(), 100);
    assert!(tree.has(&50));

    // BinaryTree2
    let mut tree = BinaryTree2::new();
    for i in 0..100 {
        tree.insert(i);
    }
    assert_eq!(tree.len(), 100);
    assert!(tree.has(&50));

    // BinaryTree3
    let mut tree = BinaryTree3::new();
    for i in 0..100 {
        tree.insert(i);
    }
    assert_eq!(tree.len(), 100);
    assert!(tree.has(&50));
}