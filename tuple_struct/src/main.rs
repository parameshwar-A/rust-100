fn main() {
    #[derive(Debug)]
    struct Meter(f64);

    #[derive(Debug)]
    struct Kilogram(f64);

    let weight: Kilogram = Kilogram(24.56);
    let length: Meter = Meter(12.5);

    println!("{:?} {:?}", weight, length);
}
