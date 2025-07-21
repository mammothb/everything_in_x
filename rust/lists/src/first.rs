#[derive(Debug, Default)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Nil }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: std::mem::replace(&mut self.head, Link::Nil),
        });
        self.head = Link::Cons(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Nil) {
            Link::Cons(node) => {
                self.head = node.next;
                Some(node.val)
            }
            Link::Nil => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr = std::mem::replace(&mut self.head, Link::Nil);
        while let Link::Cons(mut node) = curr {
            curr = std::mem::replace(&mut node.next, Link::Nil);
        }
    }
}

#[derive(Debug, Default)]
enum Link {
    Cons(Box<Node>),
    #[default]
    Nil,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_empty_list_gives_none() {
        let mut list = List::default();

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
}
