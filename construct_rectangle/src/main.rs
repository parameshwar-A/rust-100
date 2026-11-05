fn main() {

    struct Rectangle{
        width: u32,
        length: u32
    }

    impl Rectangle{
        fn new(w: u32,l:u32) -> Self{
            Rectangle{
                width: w,
                length: l
            }
        }
    }

    let val: Rectangle = Rectangle::new(10,20);
    println!("{} {}", val.length, val.width);
}
