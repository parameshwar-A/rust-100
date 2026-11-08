fn main() {
    let arr: [i32;10] = [1, 66, 22,9, 19, 29, 93, 11, 55, 21];

    let mut max: i32 = -99999999;
    let mut first_max: i32 = -99999999;

    for val in arr.iter(){

        if *val > first_max{
            max = first_max;
            first_max = *val;
        }
        else if *val < first_max && *val > max {
            max = *val;
        }
    }
    println!("First max: {} Second max: {}", first_max,max);
}
