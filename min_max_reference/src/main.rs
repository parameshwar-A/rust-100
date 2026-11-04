fn main() {

    let arr = [1,2,4,5,3, 99, 2];
    let val:(&i32,&i32) = min_max_reference(&arr);
    println!("{} {}", val.0, val.1);
}

fn min_max_reference(array: &[i32]) -> (&i32, &i32){

    let mut ind = 0;
    let length = array.len();
    let mut min_value = 99999999;
    let mut max_value = -99999999;

    let mut min_ind=7;
    let mut max_ind=0;

    while ind<length{

        if array[ind] > max_value{
            max_ind = ind;
            max_value = array[ind];
        }

        if array[ind] < min_value{
            min_ind = ind;
            min_value = array[ind];
        }

        ind+=1

    }
    (&array[min_ind], &array[max_ind])
}
