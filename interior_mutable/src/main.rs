#[derive(Debug)]
enum DynList{
    Cons(Rc<RefCell<i32>>, Rc<DynList>),
    Nil
}

use crate::DynList::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let core_value = Rc::new(RefCell::new(3));

    let a = Rc::new(Cons(Rc::clone(&core_value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(1)), Rc::clone(&a));
    
    println!("Before Mutable borrow");
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    *core_value.borrow_mut() *= 10;
    println!("After Mutable borrow");
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
