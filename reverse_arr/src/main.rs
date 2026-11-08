fn main() {

    let mut arr: [u8;10] = [1,2,3,4,5,6,7,8,9,10];

    let mut start: usize = 0;
    let mut end: usize = arr.len()-1;
    let mut temp;

    while start < end{

        temp = arr[end];
        arr[end] = arr[start];
        arr[start] = temp;
        
        start +=1;
        end-=1;
    }

    for val in arr.iter(){
        print!("{} ", val);
    }
    println!("");

}
