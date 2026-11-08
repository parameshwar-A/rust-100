fn main() {

    let arr: [u8;10] = [1,2,3,4,5,6,7,8,9,10];
    let mut even:u8 = 0;
    let mut odd:u8 = 0;

    for num in arr.iter(){

        if *num %2 ==0{
            even+=1;
        }
        else{
            odd+=1;
        }
    }

    println!(" Odd count: {} and Even Count: {}", odd, even);
}
