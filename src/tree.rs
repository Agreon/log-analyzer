use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use actix_web::{
    body::{BoxBody, MessageBody},
    error, post, web, App, HttpResponse, HttpServer, Responder,
};

/**
 * What will #inline and #must_use do?
 */
#[derive(Debug)]
struct IndexCollectionRef {
    // pub items:
    // pub indices:
}

// trait IndexCollectionLoader {}

#[derive(Debug)]
struct TreeNode {
    pub parent: Option<Weak<TreeNode>>,
    // Size of children is always >= b/2 and always <= b
    //     children: RefCell<Vec<Rc<Node>>>,
    pub children: Vec<Box<TreeNode>>,
    // The total amount of logs contained in this node and it's children.
    pub entries: u64,
    pub min: u64,
    pub max: u64,
    // TODO: The collection itself is a range again, does it change the algorithm?
    // We need to lazy load this: The value of this, is the key to load.
    pub index_collection_key: Option<String>,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            parent: None,
            // TODO: Initial capacity known thanks to param?
            children: Vec::new(),
            entries: 0,
            min: 0,
            max: 0,
            index_collection_key: None,
        }
    }

    pub fn from(children: Vec<Box<TreeNode>>) -> Self {
        let entries = children.iter().map(|c| c.entries).sum();
        let min = children[0].min;
        let max = children[children.len() - 1].max;

        TreeNode {
            parent: None,
            index_collection_key: None,
            children,
            entries,
            min,
            max,
        }
    }

    // TODO: Not generic yet, just if lowest leave
    pub fn add(&mut self, entry: LogEntry) {
        // if (self.children.capacity() > self.children.len()) {
        //      self.children.push(entry);
        // }

        // TODO: Add to index-collection, if there is space
        self.entries = self.entries + 1;
        // TODO: Increase all parents entries value
        // => seems slow, but should be ok, no comparisons necessary
        self.max = entry.time;
    }
}

struct LogEntry {
    pub time: u64,
}

struct MainTree {
    pub head: Rc<RefCell<TreeNode>>,
    // Link to right-most leave for fast insert. We expect the incoming messages to be almost always
    // somewhat in order.
    pub current_leave: Rc<RefCell<TreeNode>>,
}

impl MainTree {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(TreeNode::new()));
        let current_leave = Rc::clone(&head);

        MainTree {
            head,
            current_leave,
        }
    }

    // TODO: Move most of the logic here. We don't want recursive logic.
    pub fn insert(&self, entry: LogEntry) {
        let mut current_leave = self.current_leave.borrow_mut();
        if current_leave.min < entry.time {
            // TODO: Re-Order Tree if necessary
            current_leave.add(entry);
            return;
        }
        // TODO: Re-Order Tree if necessary
        self.head.borrow_mut().add(entry);
    }
}
