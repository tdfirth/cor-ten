use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(n) = head {
            if let Ok(mut node) = Arc::try_unwrap(n) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list
            .append(String::from("a"))
            .append(String::from("b"))
            .append(String::from("c"));
        assert_eq!(list.head(), Some(&String::from("c")));

        let list = list.tail();
        assert_eq!(list.head(), Some(&String::from("b")));

        // Append some more just to make sure nothing's corrupted
        let list = list
            .tail()
            .append(String::from("z"))
            .append(String::from("y"));
        assert_eq!(list.head(), Some(&String::from("y")));
        let list = list.tail();
        assert_eq!(list.head(), Some(&String::from("z")));
        let list = list.tail();
        assert_eq!(list.head(), Some(&String::from("a")));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
