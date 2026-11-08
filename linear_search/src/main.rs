fn main() {

    let arr: [u8;5] = [1,2,3,4,5];
    let num: u8 = 3;
    println!("Index of {} is {}", num, find(&arr, num)); 

}

fn find(arr: &[u8;5], num: u8) -> i8{
    for (ind, val) in arr.iter().enumerate(){
        if *val == num{
            return ind as i8
        }
    }
    -1
}
