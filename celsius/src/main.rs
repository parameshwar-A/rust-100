fn main() {

    let c:f32 = 28.0;

    let f: f32 = (c * (9.0/5.0)) + 32.0;

    println!("Celsius of {c} deg C is equals to {f} deg F");

    let f2: f32 = 100.0;

    let c2: f32 = (f2-32.0) * (5.0/9.0);

    println!("Farenheit of {f2} deg F is equals to {c2} deg C");

}
