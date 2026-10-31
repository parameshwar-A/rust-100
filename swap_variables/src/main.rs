fn main() {

    let mut a: i32= 5;
    let mut b: i32 = 10;
    let c: i32;

    println!("Before:");
    println!("A: {a} \nB: {b}");
    c = a;
    a = b;
    b = c;
    println!("After:");
    println!("A: {a} \nB: {b}");

}
