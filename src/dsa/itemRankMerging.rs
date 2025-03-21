use std::collections::HashMap;

#[derive(Debug)]

struct Node<T> {
    element: T,
    next: pointer<T>,
}
type pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug> {
    head: pointer<T>,
    // root: pointer<T>,
}

pub struct ItemObjectsLinkedList<T: std::fmt::Debug> {
    itemObejectList: HashMap<String, LinkedList<T>>,
    sortedListName : String
}

impl<T: std::fmt::Debug> LinkedList<T> {
    fn create_empty_list() -> LinkedList<T> {
        LinkedList {
            head: None,
            // root: None,
        }
    }

    fn add_item(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: previous_head,
        });
        self.head = Some(new_head);
        // //println!("current Head :{:?}",self.head.as_ref().unwrap().element);
    }

    fn remove_item(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        //println!("previous head: {:?} ", previous_head);
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => {
                //println!("NONE");
                None},
        }
    }

    fn printing(&self) {
        let mut list_traversal = &self.head;
        loop {
            match list_traversal {
                Some(Node) => {
                    print!("{:?}<--", Node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                }
                None => {
                    print!("NONE");
                    break;
                }
            }
        }
        println!();
    }
}

impl<T: std::fmt::Debug+Clone> ItemObjectsLinkedList<T> {
    pub fn new(sortedListName: String) -> ItemObjectsLinkedList<T> {
        ItemObjectsLinkedList {
            itemObejectList: HashMap::new(),
            sortedListName
        }
    }

    pub fn add_linked_list(&mut self, key: &str) {
        self.itemObejectList
            .insert(key.to_string(), LinkedList::create_empty_list());
    }

    pub fn upsert_item(&mut self, key: &str, value: T) {
        match self.itemObejectList.get_mut(key) {
            Some(list) => {
                // //println!("{}::{:?} ",key ,value);
                list.add_item(value);
                // list.//printing();
            }
            None => {
                self.add_linked_list(key);
                self.upsert_item(key, value);
            }
        }
    }

    pub fn remove_item(&mut self, key: &str) {
        match self.itemObejectList.get_mut(key) {
            Some(l_list) => {
                //println!("{}:: ", key);
                l_list.remove_item();
                // l_list.//printing();
                if l_list.head.is_none() {
                    self.itemObejectList.remove(key);
                }
                // l_list.//printing();
            }
            None => {
                //println!("{} No such key exists", key);
            }
        }
    }

    pub fn print_list(&self, key: &str) {
        match self.itemObejectList.get(key) {
            Some(list) => {
                print!("{}:: ", key);
                list.printing();
            }
            None => {
                println!("{} No such key exists", key);
            }
        }
    }

    pub fn get_heads(&self) -> Vec<(T, String)> {
        let mut result = Vec::new();
        for (key, list) in &self.itemObejectList {
            if *key == self.sortedListName {
                continue;
            }
            if let Some(node) = &list.head {
                result.push((node.element.clone(), key.clone()));
            }
        }
        // //println!("lengthL {:?} ", result.len());
        result
    }
}
