#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List{

    fn new() -> Self{
        List::Nil
    }

    fn prepend(self, elem: i32) -> Self {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> usize{
        let mut length: usize = 0;

        let mut current = self;
        while let List::Cons(_value, next) = current {
            length+=1;
            current = next;
        }
        length
    }

    fn stringify(&self){
        let mut current = self;
        while let List::Cons(value, next) = current {
            print!("{} ->", value);
            current = next;
        }
        println!("");
    }

}

use crate::List::{Cons, Nil};

fn main() {

    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{}", list.len());
    list = list.prepend(10);
    list.stringify();
}
