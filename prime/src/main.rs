fn main() {

    let num: u64 = 18;
    println!("is {} a prime: {}", num, is_prime(num));
}

fn is_prime(num: u64) -> bool {

    if num == 1{
        println!("The ultimate prime");
        return true
    }
    else if num == 0{
        println!("Number must be greater than 0");
        return false
    }
    else if num ==2{
        return true
    }
    else if num %2 ==0 || num %3 ==0 {
        return false
    }

    let mut val = 3;
    while val * val <= num{
        if num % val == 0 {
            return false
        }
        val +=2
    }
    return true
}
