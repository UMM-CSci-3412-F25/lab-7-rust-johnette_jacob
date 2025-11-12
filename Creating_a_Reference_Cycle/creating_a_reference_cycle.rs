use std:: cell::RefCell;
use std:: rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let b = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&a))),
    });
    // Creating a cycle: a -> b -> a
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
// Expected behavior is: a memory leak where neither a nor b wll be dropped because of the cycle
}
