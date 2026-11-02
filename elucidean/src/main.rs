fn main() {

    println!("{}", get_gcd(48, 18));
    println!("{}", get_lcm(48, 18));

}

fn get_lcm(n1: u64, n2: u64) -> u64 {

    let gcd = get_gcd(n1, n2);
    return (n1*n2)/gcd
}

fn get_gcd(n1: u64, n2: u64) -> u64 {
    let mut large;
    let mut small;

    if n1 > n2{
        large = n1;
        small = n2;
    }
    else{
        large = n2;
        small = n1;
    }

    let mut remainder=0;
    loop{
        if large % small == 0{
            break
        }
        remainder = large % small;
        large = small;
        small = remainder;
    }
    return remainder
}
