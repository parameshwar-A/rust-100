fn main() {

    let arr: [u8;10] = [1,2,3,4,5,6,7,8,9,10];

    let mut count: u8 = 0;
    let mut sum: u8 = 0;

    for num in arr.iter(){
        count+=1;
        sum += num;
    }
    let avg = sum / count;
    println!("the sum is {} and average is {}", sum, avg);
}
