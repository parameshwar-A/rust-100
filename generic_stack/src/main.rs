fn main() {

    struct Stack<T:  std::fmt::Display>{
        values: Vec<T>
    }

    impl<T:  std::fmt::Display> Stack<T>{

        fn new() -> Self {
            Stack{
                values: Vec::new()
            }
        }

        fn push(&mut self, num: T){
            self.values.push(num);
        }

        fn pop(&mut self) -> Option<T> {
            self.values.pop()
        }

        fn view(&self){
            print!("Stack: ");
            for v in &self.values{
                print!("{} ", v);
            }
            println!();
        }
    }

    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(100);

    s.view();

    let val = match s.pop() {
        Some(v) => v,
        None => 0
    };
    println!("The popped value is {}", val);
    s.view();
}
