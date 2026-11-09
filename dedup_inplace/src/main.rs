fn main() {

    let mut myvec: Vec<i32> = vec![ 10, 20, 10,  30,  30 ];
    selection_sort(&mut myvec);
    dedup(&mut myvec);
    for value in &myvec{
        print!("{}, ", value);
    }
    println!("");
}

fn dedup(v: &mut Vec<i32>){


    let length: usize = v.len();
    if length == 0 || length == 1{
        return;
    }

    let mut w: usize = 1;
    let mut r: usize = 1;

    while r < length{

        if v[r] == v[w-1]{
            r+=1;
        }
        else{
            v[w] = v[r];
            w+=1;
            r+=1;
        }
    }
    v.truncate(w);
}

fn selection_sort(ar: &mut Vec<i32>){

    let mut min_ind:usize;
    let length: usize = ar.len();
    let mut temp: i32;

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
