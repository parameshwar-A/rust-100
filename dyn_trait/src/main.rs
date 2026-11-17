fn main() {

    struct Circle{
        radius: f32
    }

    struct Rectangle{
        length: f32,
        breadth: f32
    }

    trait Area{
        fn area(&self) -> f32;
    }

    impl Area for Circle{

        fn area(&self) -> f32{
            3.14 * self.radius * self.radius
        }
    }

    impl Area for Rectangle{
        fn area(&self) -> f32{
            self.length * self.breadth
        }
    }

    let cir: Circle = Circle{ radius: 2.0 };
    let rect: Rectangle = Rectangle{length: 2.0, breadth:3.0};

    let items: Vec<Box<dyn Area>> = vec![
        Box::new(cir),
        Box::new(rect)
    ];

    for item in items.iter(){
        println!("Area is {}", item.area());
    }


}
