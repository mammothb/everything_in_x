#[derive(Debug, Default)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn push(&mut self, val: T) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_pop_gives_none() {
        let mut list: List<i32> = List::default();

        assert_eq!(None, list.pop());
    }

    #[test]
    fn push_and_pop_operations() {
        let mut list = List::default();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        list.push(4);

        assert_eq!(Some(4), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn peek() {
        let mut list = List::default();

        assert_eq!(None, list.peek());
        assert_eq!(None, list.peek_mut());

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(&3), list.peek());
        assert_eq!(Some(&mut 3), list.peek_mut());

        let _ = list.peek_mut().map(|val| *val = 4);

        assert_eq!(Some(&4), list.peek());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(None, iter.next());
    }
}
