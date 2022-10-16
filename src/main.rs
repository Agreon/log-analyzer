use std::{cell::RefCell, rc::Rc};

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

fn main() {
    let mut first_node = ListNode::new(16);
    first_node.items.push(111);
    let mut second_node: ListNode<i32> = ListNode::new(16);
    second_node.items.push(222);

    let mut l_list: UnrolledLinkedList<i32> = UnrolledLinkedList::new(first_node);

    l_list.add(second_node);

    println!("{:?}", l_list);

    println!("{:?}", l_list.first);
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, UnrolledLinkedList};

    #[test]
    fn it_works() {
        // let mut first_node = ListNode::new();
        // first_node.items.push(1);
        // let mut l_list: UnrolledLinkedList<i32> = UnrolledLinkedList::new(first_node);

        // let mut second_node: ListNode<i32> = ListNode::new();
        // second_node.items.push(2);

        // l_list.add(Box::new(second_node));

        // format!("{:?}", l_list);
    }
}
