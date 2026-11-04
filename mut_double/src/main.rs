fn main() {

    let mut a = [1,2,3,4,5];
    mut_double(&mut a);
    for val in a{
        println!("{}", val);
    }
}

fn mut_double(array: &mut [i32]){
    let mut ind = 0;
    let length = array.len();
    while ind < length{
        array[ind]= array[ind]*array[ind];
        ind+=1
    }
}
