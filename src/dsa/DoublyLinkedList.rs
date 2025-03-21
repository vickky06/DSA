use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>, // Weak to avoid reference cycles
    next: Link<T>,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T: std::fmt::Debug> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Adds a new element to the front of the list.
    ///
    /// Given a node that currently points to `old_head`, and a new node `new_node`
    /// that has `value` as its value, points `new_node` to `old_head` and points
    /// `old_head`'s predecessor to `new_node`. If `old_head` is `None`, then
    /// `new_node` is the only node in the list, so we set `self.tail` as well.
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
                self.head.as_ref().unwrap().borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    /// Adds a new element to the end of the list.
    ///
    /// Given a node that currently points to `old_tail`, and a new node `new_node`
    /// that has `value` as its value, points `old_tail` to `new_node` and points
    /// `new_node`'s predecessor to `old_tail`. If `old_tail` is `None`, then
    /// `new_node` is the only node in the list, so we set `self.head` as well.
    pub fn push_back(&mut self, value: T) -> Rc<RefCell<Node<T>>> {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: self.tail.as_ref().map(|old_tail| Rc::downgrade(old_tail)),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
        }
        new_node
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let mut old_head_borrow = old_head.borrow_mut();
            if let Some(next) = old_head_borrow.next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            drop(old_head_borrow); // Drop mutable borrow before moving old_head
            match Rc::try_unwrap(old_head) {
                Ok(node) => node.into_inner().value,
                Err(_) => {
                    // Rc still has multiple strong refs
                    println!("Cannot unwrap Rc; multiple owners detected.");
                    panic!("Unwrap failed in pop_front"); // or handle gracefully
                }
            }
        })
    }

    pub fn move_to_tail(&mut self, node: &Rc<RefCell<Node<T>>>) {
        if self.tail.is_none() {
            // Defensive: If tail is None, either list is empty or corrupted
            println!("Warning: move_to_tail called but tail is None. Aborting move.");
            return;
        }

        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        match (prev, next) {
            (None, None) => {
                // Only node in list
                return;
            }
            (Some(_), None) => {
                // Already at tail
                return;
            }
            (None, Some(next_rc)) => {
                // Node is head
                next_rc.borrow_mut().prev = None;
                node.borrow_mut().next = None;
                self.head = Some(next_rc.clone());

                if let Some(prev_tail) = self.tail.as_ref() {
                    prev_tail.borrow_mut().next = Some(node.clone());
                    node.borrow_mut().prev = Some(Rc::downgrade(prev_tail));
                    self.tail = Some(node.clone());
                } else {
                    println!("Error: tail unexpectedly None in (head, next) case.");
                }
            }
            (Some(prev_weak), Some(next_rc)) => {
                // Node is in middle
                if let Some(prev_rc) = prev_weak.upgrade() {
                    prev_rc.borrow_mut().next = Some(next_rc.clone());
                    next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));

                    node.borrow_mut().prev = None;
                    node.borrow_mut().next = None;

                    if let Some(prev_tail) = self.tail.as_ref() {
                        prev_tail.borrow_mut().next = Some(node.clone());
                        node.borrow_mut().prev = Some(Rc::downgrade(prev_tail));
                        self.tail = Some(node.clone());
                    } else {
                        println!("Error: tail unexpectedly None in middle case.");
                    }
                }
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            let mut old_tail_borrow = old_tail.borrow_mut();
            if let Some(prev_weak) = old_tail_borrow.prev.take() {
                if let Some(prev) = prev_weak.upgrade() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                }
            } else {
                self.head.take();
            }
            drop(old_tail_borrow); //  mutable borrow drop before moving old_tail
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn print_forward(&self) {
        println!("Forward traversal:");
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:?} ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!();
    }

    pub fn print_backward(&self) {
        println!("Backward traversal:");
        let mut current = self.tail.clone();
        while let Some(node) = current {
            print!("{:?} ", node.borrow().value);
            if let Some(prev_weak) = &node.borrow().prev {
                current = prev_weak.upgrade();
            } else {
                current = None;
            }
        }
        println!();
    }
}
