/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/



use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord + std::cmp::PartialEq + Clone> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        if let Some(root) = &mut self.root {
            if root.value == value {
                return;
            }
            root.insert(value);
        }
        
    }


    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if let Some(root) = &self.root {
            if root.value == value {
                return true;
            }
            return Self::check_children(root, &value);
        }
        return false;
    }

    fn check_children(node: &TreeNode<T>, value: &T) -> bool {
        if let Some(node_l) = &node.left {
            if &node_l.value == value {
                return true;
            }
            if let Some(node_r) = &node.right {
                if &node_r.value == value {
                    return true;
                }
                return Self::check_children(&node_l, value) || Self::check_children(&node_r, value);
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if self.value == value {
            return;
        } else {
            if self.value > value {
                if let None = self.left {
                    self.left = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                if let Some(node) = &mut self.left {
                    if node.value == value {
                        return;
                    }
                    node.insert(value);
                    return;
                }
            } else {
                if let None = self.right {
                    self.right = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                if let Some(node) = &mut self.right {
                    if node.value == value {
                        return;
                    }
                    node.insert(value);
                    return;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


