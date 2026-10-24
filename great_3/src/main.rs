fn main() {

    let a: i32 = 8;
    let b: i32 = 3;
    let c: i32 = 9;

    if a>b && a>c {
        println!("A is Greatest");
    }
    else if b>a && b>c {
        println!("B is Greatest");
    }
    else {
        println!("C is Greatest");
    }
}
