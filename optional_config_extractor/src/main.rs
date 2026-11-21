fn main() {

    let a: String = String::from("a");

    let config: i32 = a.parse().unwrap_or_else(|_| {
        10
    });

    println!("Config value: {}", config);

}
