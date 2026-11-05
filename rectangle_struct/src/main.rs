fn main() {

    struct Rectangle{
        width: i32,
        length: i32
    }

    let value: Rectangle = Rectangle{
        width: 20,
        length: 20
    };

    println!("{} {}", value.width, value.length);
    println!("{} {}", value.area(), value.perimeter());

    impl Rectangle{

        fn area(&self) -> i32{
            self.width * self.length
        }

        fn perimeter(&self) -> i32{
            (2 * self.width)+ (2*self.length)
        }
 
    }
    

}
