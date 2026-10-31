fn main() {

    let mut num: i32 = 121;
    let original: i32 = num;
    let mut digit:i32;
    let mut result:i32 = 0;

    while num>0{
        digit = num % 10;
        result = result * 10 + digit;
        num = num / 10;
    }

    if original == result{
        println!("{result} is a palindrome");
        }
    else {
        println!("{result} is not a palindrome");
    }
}

