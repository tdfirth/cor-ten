#![allow(dead_code)]
use std::cmp::Ordering;

const B: usize = 4;

pub struct BinarySearchTree<K, V> {
    root: Option<Box<Node<K, V>>>,
    length: u64,
}

impl<K: Ord, V> BinarySearchTree<K, V> {
    pub fn new() -> BinarySearchTree<K, V> {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    pub fn find(&self, key: K) -> Option<&V> {
        // let mut cur_node = &self.root;
        match &self.root {
            None => None,
            Some(n) => n.find(key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match &mut self.root {
            None => self.root = Some(Node::new(key, value)),
            Some(n) => n.insert(key, value),
        }

        self.length += 1;
    }

    pub fn delete(&mut self, key: K, value: V) {
        match &mut self.root {
            None => self.root = Some(Node::new(key, value)),
            Some(n) => n.insert(key, value),
        }

        self.length -= 1;
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Box<Node<K, V>> {
        Box::new(Node {
            key: key,
            value: value,
            left: None,
            right: None,
        })
    }

    pub fn find(&self, key: K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => match &self.left {
                None => None,
                Some(n) => n.find(key),
            },
            Ordering::Equal => Some(&self.value),
            Ordering::Greater => match &self.right {
                None => None,
                Some(n) => n.find(key),
            },
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            Ordering::Less => match &mut self.left {
                None => self.left = Some(Node::new(key, value)),
                Some(n) => n.insert(key, value),
            },
            Ordering::Equal => self.value = value,
            Ordering::Greater => match &mut self.right {
                None => self.right = Some(Node::new(key, value)),
                Some(n) => n.insert(key, value),
            },
        }
    }

    pub fn delete(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            Ordering::Less => match &mut self.left {
                None => (),
                Some(n) => n.delete(key, value),
            },
            Ordering::Equal => self.value = value,
            Ordering::Greater => match &mut self.right {
                None => (),
                Some(n) => n.delete(key, value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_insert_and_find() {
        let mut tree: BinarySearchTree<i64, i64> = BinarySearchTree::new();
        tree.insert(8, 16);
        tree.insert(4, 8);
        tree.insert(10, 20);
        tree.insert(6, 12);
        tree.insert(12, 24);

        assert_eq!(tree.length, 5);

        for i in vec![8, 4, 10, 6, 12] {
            match tree.find(i as i64) {
                None => println!(":("),
                Some(v) => assert_eq!(*v, i * 2),
            }
        }
    }

    #[test]
    fn test_insert_and_find_string_key() {
        let mut tree: BinarySearchTree<String, i64> = BinarySearchTree::new();
        tree.insert(String::from("hello"), 4);
        assert_eq!(*tree.find(String::from("hello")).unwrap(), 4)
    }

    #[test]
    fn test_insert_and_find_string_value() {
        let mut tree: BinarySearchTree<i64, String> = BinarySearchTree::new();
        tree.insert(4, String::from("hello"));
        assert_eq!(*tree.find(4).unwrap(), String::from("hello"))
    }

    fn assert_left_lt_key_lt_right<K: Ord, V>(node: &Node<K, V>) -> bool {
        match (&node.left, &node.right) {
            (None, None) => true,
            (Some(n), None) => n.key < node.key && assert_left_lt_key_lt_right(n),
            (None, Some(n)) => node.key < n.key && assert_left_lt_key_lt_right(n),
            (Some(n), Some(m)) => {
                n.key < node.key
                    && node.key < m.key
                    && assert_left_lt_key_lt_right(n)
                    && assert_left_lt_key_lt_right(m)
            }
        }
    }

    #[test]
    fn test_insert_order() {
        let mut tree: BinarySearchTree<i64, i64> = BinarySearchTree::new();
        tree.insert(8, 16);
        tree.insert(4, 8);
        tree.insert(10, 20);
        tree.insert(6, 12);
        tree.insert(12, 24);
        assert_eq!(assert_left_lt_key_lt_right(&tree.root.unwrap()), true);
    }
}
