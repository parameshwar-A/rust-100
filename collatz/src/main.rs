fn main() {

    let mut track: i32 = 0;
    let mut number: i32 = 6;

    print!("{}", number);
    while number !=1 {

        if number %2 == 0{
            number = number/2;
        }

        else{
            number = (3 * number) + 1;
        }
        track+=1;
        print!("-> {} ", number);
    }

    println!("\nTotal runs: {}", track);
}
