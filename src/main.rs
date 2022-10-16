use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/**
 * What will #inline and #must_use do?
 */

#[derive(Debug)]
struct ListNode<T> {
    pub items: Vec<T>,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(size: usize) -> Self {
        ListNode {
            items: Vec::with_capacity(size),
            next: None,
        }
    }

    fn set_next(&mut self, item: Rc<RefCell<ListNode<T>>>) {
        self.next = Some(item);
    }
}

#[derive(Debug)]
struct UnrolledLinkedList<T> {
    first: Rc<RefCell<ListNode<T>>>,
    last: Rc<RefCell<ListNode<T>>>,
}

impl<T> UnrolledLinkedList<T> {
    pub fn new(first_item: ListNode<T>) -> Self {
        let first = Rc::new(RefCell::new(first_item));
        let last = Rc::clone(&first);
        UnrolledLinkedList { first, last }
    }

    pub fn add(&mut self, item: ListNode<T>) {
        let point = Rc::new(RefCell::new(item));
        let new_last = point.clone();

        self.last.borrow_mut().set_next(point);
        self.last = new_last;
    }
}

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
    // TODO: optional Pointer to Index-Collection
    // => The collection itself is a range again
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            parent: None,
            children: Vec::new(),
            entries: 0,
            min: 0,
            max: 0,
        }
    }

    pub fn from(children: Vec<Box<TreeNode>>) -> Self {
        let entries = children.iter().map(|c| c.entries).sum();
        let min = children[0].min;
        let max = children[children.len() - 1].max;

        TreeNode {
            parent: None,
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

struct Tree {
    pub head: Rc<RefCell<TreeNode>>,
    // TODO: Add link to right-most leave
    pub current_leave: Rc<RefCell<TreeNode>>,
}

impl Tree {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(TreeNode::new()));
        let current_leave = Rc::clone(&head);

        Tree {
            head,
            current_leave,
        }
    }

    pub fn add(&self, entry: LogEntry) {
        self.current_leave.borrow_mut().add(entry)
    }
}

fn main() {
    let mut first_node = ListNode::new(16);
    first_node.items.push(111);
    let mut second_node: ListNode<i32> = ListNode::new(16);
    second_node.items.push(222);

    let mut l_list: UnrolledLinkedList<i32> = UnrolledLinkedList::new(first_node);

    l_list.add(second_node);

    println!("{:?}", l_list);

    println!("{:?}", l_list.first);

    let entries = [
        LogEntry {
            time: 1665622800655,
        },
        LogEntry {
            time: 1665622800656,
        },
        LogEntry {
            time: 1665622800657,
        },
    ];

    let tree = Tree::new();
    tree.add(LogEntry {
        time: 1665622800655,
    });
}

// #[cfg(test)]
// mod tests {
//     use crate::{ListNode, UnrolledLinkedList};

//     #[test]
//     fn it_works() {
//         let mut first_node = ListNode::new(16);
//         first_node.items.push(111);
//         let mut second_node: ListNode<i32> = ListNode::new(16);
//         second_node.items.push(222);

//         let mut l_list: UnrolledLinkedList<i32> = UnrolledLinkedList::new(first_node);

//         l_list.add(second_node);

//         // assert_eq!(l_list.first, l_list.last);

//         // format!("{:?}", l_list);
//     }
// }
