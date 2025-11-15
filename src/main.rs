use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, Weak},
};

pub struct Node<T> {
    value: T,
    prev: Option<Weak<Mutex<Node<T>>>>,
    next: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: std::fmt::Display> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
    length: usize,
}

impl<T: std::fmt::Display> std::fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "[]")
        } else {
            write!(f, "[")?;
            self.iter().enumerate().for_each(|(i, n)| {
                write!(f, "{}{}", n, if i < self.length - 1 { ", " } else { "" }).unwrap()
            });
            write!(f, "]")
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_head(&mut self, value: T) {
        if self.head.is_none() {
            let node = Some(Arc::new(Mutex::new(Node {
                value,
                prev: None,
                next: None,
            })));
            self.head = node.clone();
            self.tail = node;
            self.length = 1;
        } else {
            let node = Some(Arc::new(Mutex::new(Node {
                value,
                prev: None,
                next: self.head.clone(),
            })));
            self.head.as_ref().map(|n| {
                n.lock()
                    .map(|mut n1| {
                        n1.prev = node.as_ref().map(Arc::downgrade).clone();
                    })
                    .unwrap_or_else(|e| {
                        eprintln!("Oh no! Something wrong here: {}", e);
                    })
            });
            self.head = node;
            self.length += 1;
        }
    }

    pub fn push_tail(&mut self, value: T) {
        if self.tail.is_none() {
            let node = Some(Arc::new(Mutex::new(Node {
                value,
                prev: None,
                next: None,
            })));
            self.head = node.clone();
            self.tail = node;
            self.length = 1;
        } else {
            let node = Some(Arc::new(Mutex::new(Node {
                value,
                prev: self.tail.as_ref().map(Arc::downgrade).clone(),
                next: None,
            })));
            self.tail.as_ref().map(|n| {
                n.lock()
                    .map(|mut n1| {
                        n1.next = node.clone();
                    })
                    .unwrap_or_else(|e| {
                        eprintln!("Oh no! Something wrong here: {}", e);
                    })
            });
            self.tail = node;
            self.length += 1;
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.clone(),
            _marker: PhantomData,
        }
    }
}

pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

pub struct Iter<'a, T> {
    current: Option<Arc<Mutex<Node<T>>>>,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    current: Option<Arc<Mutex<Node<T>>>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.head.is_none() {
            None
        } else {
            let mut head = self.list.head.take();
            head.as_ref().map(|n| {
                n.lock()
                    .map(|n1| self.list.head = n1.next.clone())
                    .unwrap_or_else(|e| {
                        eprintln!("Oh no! Something wrong here: {}", e);
                    })
            });
            if self.list.head.is_none() {
                None
            } else {
                head.take()
                    .and_then(|n| Arc::try_unwrap(n).ok())
                    .and_then(|n| n.into_inner().ok())
                    .map(|n| n.value)
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        self.current = current.as_ref().lock().ok()?.next.clone();
        Some(unsafe { &*(&current.as_ref().lock().ok()?.value as *const T) })
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_head(23);
    list.push_head(24);
    list.push_head(25);
    list.push_tail(33);
    list.push_tail(34);
    list.push_tail(35);
    println!("{}", list);
}
