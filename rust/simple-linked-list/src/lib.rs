use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            self.head = head.next.take();
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|ref node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> 
        where T: Copy 
    {
        let vec: Vec<_> = self.into();
        
        vec.iter().rev().fold(SimpleLinkedList::new(), |mut acc, i| {
            acc.push(*i);
            acc
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}


impl<T> Default for SimpleLinkedList<T> {
    fn default() -> SimpleLinkedList<T> {
        SimpleLinkedList::new()
    }
}


impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter.into_iter().fold(SimpleLinkedList::new(), |mut acc, i| {
            acc.push(i);
            acc
        })
    }
}


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            self.next = next.next.as_deref();
            &next.data
        })
    }
}


pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            self.next = next.next.as_deref_mut();
            &mut next.data
        })
    }
}


pub struct IntoIter<T> {
    list: SimpleLinkedList<T>,
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}


impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec: Vec<_> = self.into_iter().collect();

        vec.reverse();
        vec
    }
}
