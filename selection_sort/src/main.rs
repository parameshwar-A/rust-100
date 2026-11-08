fn main() {
    let mut arr:[u8;5] = [10,2,9,4,1];
    selection_sort(&mut arr);
    for val in arr.iter(){
        print!("{} ", val);
    }
    println!("");
}

fn selection_sort(ar: &mut [u8;5]){

    let mut min_ind:usize;
    let length: usize = ar.len();
    let mut temp: u8;

    for i in 0..=length-1{

        min_ind = i;

        for j in i+1..=length-1{

            if ar[j]<ar[min_ind]{
                min_ind = j;
            }

        }
        temp = ar[min_ind];
        ar[min_ind] = ar[i];
        ar[i] = temp;
    }
}
