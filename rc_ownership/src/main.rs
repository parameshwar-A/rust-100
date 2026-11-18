enum List{
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    let c = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));
    println!("Right after creation: Count: {}", Rc::strong_count(&c));
    let a = Cons(1, Rc::clone(&c));
    println!("Right after adding a: Count: {}", Rc::strong_count(&c));
    {
        let b = Cons(2, Rc::clone(&c));
        println!("Inside block: Right after adding b: Count: {}", Rc::strong_count(&c));
    }
        println!("Outside block: Right after b is out of scope: Count: {}", Rc::strong_count(&c));


}
