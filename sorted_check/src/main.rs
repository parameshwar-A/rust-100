fn main() {

    let arr: [u8;10] = [1,20,3,4,5,6,7,8,9,10];
    println!("Is Sorted: {}", is_sorted(&arr));
}

fn is_sorted(array: &[u8;10]) -> bool {
    let mut ind:usize = 0;
    while ind < array.len()-1 {
        if array[ind] > array[ind+1]{
            return false
        }
        ind+=1
    }
   true 
}
