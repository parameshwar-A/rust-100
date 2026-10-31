fn main() {

    let mut num: i32 = 1234567;
    let mut digit: i32;
    let mut result: i32 = 0;

    while num > 0{

        digit = num % 10;
        result = result * 10 + digit;
        num = num / 10;

    }
    println!("Reverse is: {result}");
}
