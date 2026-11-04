fn main() {

    let a = [1, 2, 3, 4, 5];
    let val = slice_sum(&a[0..4]);
    println!("{}", val);

}

fn slice_sum(slice: &[i32]) -> i32 {

    let mut sum:i32 = 0;

    for val in slice {
        sum += val;
    }

    sum
}
