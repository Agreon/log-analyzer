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
