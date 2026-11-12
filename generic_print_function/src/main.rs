fn main() {
    let a: i32 = 10;
    print_me(&a);

    let b: f32 = 3.0;
    print_me(&b);

    let c: bool = true;
    print_me(&c);

    let d: String = String::from("Welcome");
    print_me(&d);
}

fn print_me<T: std::fmt::Display> (item: &T) {

    println!("{}", item);
}
