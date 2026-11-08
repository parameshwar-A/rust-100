fn main() {
    enum Shape{
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64)
    }

    let cir: Shape = Shape::Circle(2.0);
    let rect: Shape = Shape::Rectangle(3.0, 4.0);
    let tri: Shape = Shape::Triangle(2.0, 3.0);
    let shapes= [cir, rect, tri];
    fn area_circle(radius:&f64) -> f64 {
        3.14 * radius * radius
    }

    fn area_rectangle(length:&f64, width: &f64) -> f64{
        length*width
    }

    fn area_triangle(height: &f64, base:&f64) -> f64 {
        (height*base)/2.0
    }

    for val in shapes.iter(){
    match val {
        Shape::Circle(r) => {
            let area: f64 = area_circle(r);
            println!("The area of circle with radius {} is {}", r, area);
        },
        Shape::Rectangle(l,w) => {
            let area: f64 = area_rectangle(l,w);
            println!("The area of rectangle with length: {} and breadth: {} is {}", l,w, area);
        },
        Shape::Triangle(h,b) => {
            let area: f64 = area_triangle(h,b);
            println!("The area of triangle with height of base: {} and base length: {} is {}", h,b, area);
        },
    }
    }
}
