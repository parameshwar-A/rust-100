fn main() {
    for n in (1..100){
        if is_prime(n){
            println!("{}", n);
        }
    }
}

fn is_prime(num: u64)-> bool{

    if num <1 {
        return false
    }
    else if num <=3{
        return true
    }
    else if num %2 ==0 || num %3 == 0 {
        return false 
    }
    
    let mut val = 3;
    while val * val <= num{
        if num % val == 0 {
            return false
        }
        val += 2
    }
    return true
}
