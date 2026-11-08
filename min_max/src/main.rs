fn main() {

    let arr: [i32;10] = [2,7,2,0,10, 88, 39,5, 22,11];

    let mut min: i32 = 999999999;
    let mut max: i32 = -99999999;

    for val in arr.iter(){
        if *val< min{
            min = *val;
        }
        if *val> max{
            max = *val;
        }
    }

    println!("min: {} max: {}", min, max);
}
