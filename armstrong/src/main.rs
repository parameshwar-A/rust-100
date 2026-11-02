fn main() {

    let num = 134;
    println!("Is {} armstrong ? {}",num, is_armstrong(num));
}

fn is_armstrong(num: u64) -> bool {

    let mut val = num;

    let mut count = 0;
    while val > 0{
        val = val / 10;
        count+=1
    }

    let mut val2 = num;
    let mut digit;
    let mut result = 0;
    while val2 > 0{
        digit = val2 %10;
        result = result + digit.pow(count);
        val2 = val2/10;
    }

    if num == result{
        return true
    }
    return false
}
