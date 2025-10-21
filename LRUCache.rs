use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::new(-1, -1);
        let tail = Node::new(-1, -1);
        
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }
    
    fn add_node(&mut self, node: &Rc<RefCell<Node>>) {
        let head_next = self.head.borrow().next.clone();
        
        node.borrow_mut().next = head_next.clone();
        node.borrow_mut().prev = Some(Rc::clone(&self.head));
        
        if let Some(ref next) = head_next {
            next.borrow_mut().prev = Some(Rc::clone(node));
        }
        
        self.head.borrow_mut().next = Some(Rc::clone(node));
    }
    
    fn delete_node(&mut self, node: &Rc<RefCell<Node>>) {
        let prev_node = node.borrow().prev.clone();
        let next_node = node.borrow().next.clone();
        
        if let Some(ref prev) = prev_node {
            prev.borrow_mut().next = next_node.clone();
        }
        
        if let Some(ref next) = next_node {
            next.borrow_mut().prev = prev_node.clone();
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let result = node.borrow().value;
            let node_clone = Rc::clone(node);
            
            self.delete_node(&node_clone);
            self.add_node(&node_clone);
            
            result
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            let node_clone = Rc::clone(node);
            node_clone.borrow_mut().value = value;
            self.delete_node(&node_clone);
            self.add_node(&node_clone);
        } else {
            if self.map.len() >= self.capacity {
                let lru_node = self.tail.borrow().prev.clone();
                
                if let Some(ref lru) = lru_node {
                    let lru_key = lru.borrow().key;
                    if lru_key != -1 {
                        self.map.remove(&lru_key);
                        self.delete_node(&lru);
                    }
                }
            }
            
            let new_node = Node::new(key, value);
            self.add_node(&new_node);
            self.map.insert(key, new_node);
        }
    }
}

