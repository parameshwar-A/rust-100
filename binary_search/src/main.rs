fn main() {

    let arr: [u8;10] = [1,2,3,4,5,6,7,8,9,10];

    println!("The index of value is {}", binary_search(&arr, 40));


}

fn binary_search(ar: &[u8;10], val: u8) -> i8 {

    let mut low:usize = 0;
    let mut high:usize = ar.len()-1;
    let mut mid: usize;

    while low <= high{

        mid = low + (high-low)/2;

        if ar[mid] == val{
            return mid as i8;
        }

        else if ar[mid] > val{
            high = mid -1;
        }

        else if ar[mid] < val{
            low = mid +1;
        }
    }
    -1
}
