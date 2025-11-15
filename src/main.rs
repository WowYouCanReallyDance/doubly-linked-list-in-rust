use std::{fmt::Display, marker::PhantomData};

struct Node<T> {
    value: T,
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push_head(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            prev: None,
            next: self.head,
        });
        let node_ptr = Box::into_raw(node);

        if let Some(head) = self.head {
            unsafe {
                (*head).prev = Some(node_ptr);
            }
        } else {
            self.tail = Some(node_ptr);
        }

        self.head = Some(node_ptr);
        self.length += 1;
    }

    pub fn push_tail(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            prev: self.tail,
            next: None,
        });
        let node_ptr = Box::into_raw(node);

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(node_ptr);
            }
        } else {
            self.tail = Some(node_ptr);
        }

        self.tail = Some(node_ptr);
        self.length += 1;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

pub struct Iter<'a, T> {
    current: Option<*mut Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node_ptr = self.current?;
        self.current = (unsafe { &*current_node_ptr }).next;
        Some(&(unsafe { &*current_node_ptr }).value)
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: Display> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, ele) in self.iter().enumerate() {
            write!(f, "{}{}", ele, if i < self.len() - 1 { ", " } else { "" })?;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_head(23);
    list.push_head(24);
    list.push_head(25);
    list.push_tail(26);
    list.push_tail(27);
    list.push_tail(28);
    println!("{}", list);

    let mut list = DoublyLinkedList::new();
    list.push_head("AAA");
    list.push_head("BBB");
    list.push_head("CCC");
    list.push_tail("DDD");
    list.push_tail("EEE");
    list.push_tail("FFF");
    println!("{}", list);
}
