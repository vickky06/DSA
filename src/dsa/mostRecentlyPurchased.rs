use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::dsa::DoublyLinkedList::DoublyLinkedList;

use super::DoublyLinkedList::Node;

// use super::DoublyLinkedList::Link;

pub struct Item_Map {
    map: HashMap<i32, Rc<RefCell<Node<i32>>>>,
    item_list: DoublyLinkedList<i32>,
    capacity: i32,
    size: i32,
}
pub enum Direction {
    Forward,
    Backward,
}

impl Item_Map {
    pub fn new(capacity: i32) -> Self {
        Item_Map {
            map: HashMap::new(),
            item_list: DoublyLinkedList::new(),
            capacity,
            size: 0,
        }
    }

    pub fn purchased(&mut self, item: i32) {
        if let Some(node_rc) = self.map.get(&item) {
            self.item_list.move_to_tail(node_rc);
        } else {
            if self.size >= self.capacity {
                if let Some(removed_item) = self.item_list.pop_front() {
                    self.map.remove(&removed_item);
                    self.size -= 1;
                }
            }
            let node = self.item_list.push_back(item);
            self.map.insert(item, node);
            self.size += 1;
        }
    }

    pub fn print_list(&self, direction: Direction) {
        match direction {
            Direction::Backward => self.item_list.print_backward(),
            Direction::Forward => self.item_list.print_forward(),
        }
    }
}
