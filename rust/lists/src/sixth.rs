use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    io::Cursor,
    marker::PhantomData,
    ptr::NonNull,
};

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _dummy: PhantomData<T>,
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

pub struct Iter<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _dummy: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _dummy: PhantomData<&'a mut T>,
}

pub struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    cur: Link<T>,
    index: Option<usize>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    val: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _dummy: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        while self.head.is_some() {
            self.pop_front();
        }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| &(*node.as_ptr()).val) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.map(|node| &mut (*node.as_ptr()).val) }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.map(|node| &(*node.as_ptr()).val) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.map(|node| &mut (*node.as_ptr()).val) }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            list: self,
            cur: None,
            index: None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _dummy: PhantomData,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _dummy: PhantomData,
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.tail.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());

                self.tail = boxed_node.prev;
                if let Some(new_tail) = self.tail {
                    (*new_tail.as_ptr()).next = None;
                } else {
                    self.head = None;
                }

                self.len -= 1;
                boxed_node.val
            })
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());

                self.head = boxed_node.next;
                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).prev = None;
                } else {
                    self.tail = None;
                }

                self.len -= 1;
                boxed_node.val
            })
        }
    }

    pub fn push_back(&mut self, val: T) {
        unsafe {
            let new_tail = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                prev: None,
                next: None,
                val,
            })));
            if let Some(old_tail) = self.tail {
                (*old_tail.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old_tail);
            } else {
                self.head = Some(new_tail);
            }
            self.tail = Some(new_tail);
            self.len += 1;
        }
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                prev: None,
                next: None,
                val,
            })));
            if let Some(old_head) = self.head {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            } else {
                self.tail = Some(new_head);
            }
            self.head = Some(new_head);
            self.len += 1;
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            self.pop_front();
        }
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|node| unsafe {
                self.len -= 1;
                self.head = (*node.as_ptr()).next;
                &(*node.as_ptr()).val
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.tail.map(|node| unsafe {
                self.len -= 1;
                self.tail = (*node.as_ptr()).prev;
                &(*node.as_ptr()).val
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|node| unsafe {
                self.len -= 1;
                self.head = (*node.as_ptr()).next;
                &mut (*node.as_ptr()).val
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.tail.map(|node| unsafe {
                self.len -= 1;
                self.tail = (*node.as_ptr()).prev;
                &mut (*node.as_ptr()).val
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

impl<'a, T> CursorMut<'a, T> {
    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).val) }
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                self.cur = (*cur.as_ptr()).prev;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.cur = self.list.tail;
            self.index = Some(self.list.len - 1);
        }
    }

    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                self.cur = (*cur.as_ptr()).next;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            self.cur = self.list.head;
            self.index = Some(0);
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cur) = self.cur {
                (*cur.as_ptr()).prev
            } else {
                self.list.tail
            };
            prev.map(|node| &mut (*node.as_ptr()).val)
        }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(cur) = self.cur {
                (*cur.as_ptr()).next
            } else {
                self.list.head
            };
            next.map(|node| &mut (*node.as_ptr()).val)
        }
    }

    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
            } else if let Some(cur) = self.cur {
                let in_head = input.head.take().unwrap();
                let in_tail = input.tail.take().unwrap();

                if let Some(next) = (*cur.as_ptr()).next {
                    (*next.as_ptr()).prev = Some(in_tail);
                    (*in_tail.as_ptr()).next = Some(next);
                    (*cur.as_ptr()).next = Some(in_head);
                    (*in_head.as_ptr()).prev = Some(cur);
                } else {
                    (*cur.as_ptr()).next = Some(in_head);
                    (*in_head.as_ptr()).prev = Some(cur);
                    self.list.tail = Some(in_tail);
                }
            } else if let Some(head) = self.list.head {
                let in_head = input.head.take().unwrap();
                let in_tail = input.tail.take().unwrap();

                (*head.as_ptr()).prev = Some(in_tail);
                (*in_tail.as_ptr()).next = Some(head);
                self.list.head = Some(in_head);
            } else {
                std::mem::swap(self.list, &mut input);
            }
            self.list.len += input.len;
            input.len = 0;
        }
    }

    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
            } else if let Some(cur) = self.cur {
                let in_head = input.head.take().unwrap();
                let in_tail = input.tail.take().unwrap();

                if let Some(prev) = (*cur.as_ptr()).prev {
                    (*prev.as_ptr()).next = Some(in_head);
                    (*in_head.as_ptr()).prev = Some(prev);
                    (*cur.as_ptr()).prev = Some(in_tail);
                    (*in_tail.as_ptr()).next = Some(cur);
                } else {
                    (*cur.as_ptr()).prev = Some(in_tail);
                    (*in_tail.as_ptr()).next = Some(cur);
                    self.list.head = Some(in_head);
                }
                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(tail) = self.list.tail {
                let in_head = input.head.take().unwrap();
                let in_tail = input.tail.take().unwrap();

                (*tail.as_ptr()).next = Some(in_head);
                (*in_head.as_ptr()).prev = Some(tail);
                self.list.tail = Some(in_tail);
            } else {
                std::mem::swap(self.list, &mut input);
            }
            self.list.len += input.len;
            input.len = 0;
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            unsafe {
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let next = (*cur.as_ptr()).next;

                let new_len = old_idx + 1;
                let new_tail = self.cur;
                let new_head = self.list.head;
                let new_idx = Some(old_idx);

                let output_len = old_len - new_len;
                let output_head = next;
                let output_tail = self.list.tail;

                if let Some(next) = next {
                    (*cur.as_ptr()).next = None;
                    (*next.as_ptr()).prev = None;
                }

                self.list.len = new_len;
                self.list.head = new_head;
                self.list.tail = new_tail;
                self.index = new_idx;

                LinkedList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    _dummy: PhantomData,
                }
            }
        } else {
            std::mem::take(self.list)
        }
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            unsafe {
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let prev = (*cur.as_ptr()).prev;

                let new_len = old_len - old_idx;
                let new_head = self.cur;
                let new_tail = self.list.tail;
                let new_idx = Some(0);

                let output_len = old_len - new_len;
                let output_head = self.list.head;
                let output_tail = prev;

                if let Some(prev) = prev {
                    (*cur.as_ptr()).prev = None;
                    (*prev.as_ptr()).next = None;
                }

                self.list.len = new_len;
                self.list.head = new_head;
                self.list.tail = new_tail;
                self.index = new_idx;

                LinkedList {
                    head: output_head,
                    tail: output_tail,
                    len: output_len,
                    _dummy: PhantomData,
                }
            }
        } else {
            std::mem::take(self.list)
        }
    }
}

#[allow(dead_code)]
fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();

    is_send::<IntoIter<i32>>();
    is_sync::<IntoIter<i32>>();

    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    is_send::<IterMut<i32>>();
    is_sync::<IterMut<i32>>();

    is_send::<Cursor<i32>>();
    is_sync::<Cursor<i32>>();

    fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> {
        x
    }

    fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> {
        x
    }

    fn into_iter_covariant<'a, T>(x: IntoIter<&'static T>) -> IntoIter<&'a T> {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next_back(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = f64::NAN;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{list:?}"), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = ["just", "one", "test", "more"].iter().copied().collect();
        assert_eq!(format!("{list:?}"), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }

    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));

        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }

    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.splice_before(Some(7).into_iter().collect());
        cursor.splice_after(Some(8).into_iter().collect());
        // check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[7, 1, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
        );

        /* remove_current not impl'd
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(7));
        cursor.move_prev();
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), Some(9));
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(10));
        check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[1, 8, 2, 3, 4, 5, 6]);
        */

        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 8, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        let tmp = cursor.split_after();
        assert_eq!(
            tmp.into_iter().collect::<Vec<_>>(),
            &[102, 103, 8, 2, 3, 4, 5, 6]
        );
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101]
        );
    }

    fn check_links<T: Eq + std::fmt::Debug>(list: &LinkedList<T>) {
        let from_front: Vec<_> = list.iter().collect();
        let from_back: Vec<_> = list.iter().rev().collect();
        let re_reved: Vec<_> = from_back.into_iter().rev().collect();

        assert_eq!(from_front, re_reved);
    }
}
