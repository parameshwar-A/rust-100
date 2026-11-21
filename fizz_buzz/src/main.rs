fn main() {
    for value in 1..=100{

        if value % 3==0 && value % 5 ==0{
            println!("FizzBuzz");
        }
        else if value %3 ==0 {
            println!("Fizz");
        }
        else if value %5 ==0 {
            println!("Buzz");
        }
    }
}
