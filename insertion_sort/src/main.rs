fn main() {
    let mut arr:[u8;5] = [10,2,5,1,99];
    insertion_sort(&mut arr);
    for val in arr.iter(){
        print!("{} ", val);
    }
    println!("");
}

fn insertion_sort(ar: &mut [u8;5]){
    let length: usize = ar.len();
    let mut j: usize;
    let mut key: u8;
    for i in 1..length{

        key = ar[i];
        j = i;

        while j >0 && ar[j-1]> key{
            ar[j] = ar[j-1];
            j -= 1;
        }
        ar[j]=key;
    }
}
