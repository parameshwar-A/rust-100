fn main() {

    let mut arr:[u8;5] = [200,1,50,8,2];
    bubble_sort(&mut arr);
    for val in arr.iter(){
        print!("{} ", val);
    }
    println!("");
}

fn bubble_sort(ar: &mut [u8;5]){

    let length: usize = ar.len();
    let mut swapped: bool;
    let mut temp: u8;

    for i in 0..=length-1{
        swapped = false;

        for j in 0..=length-i-2{

            if ar[j] > ar[j+1]{
                temp = ar[j+1];
                ar[j+1] = ar[j];
                ar[j] = temp;
                swapped = true;
            }
        }
        if swapped == false{
            break;
        }
    }
}
