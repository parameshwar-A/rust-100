fn main() {

    let mut a: i32 = 5;
    let mut fact: i32 = 1;
    let b: i32 = a;

    while a>0 {
        fact = fact * a;
        a = a-1;
    }
    println!("The factorial of {b} is {fact}");
}
