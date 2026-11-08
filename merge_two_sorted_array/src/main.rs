fn main() {
    let arr1:[u8;5] = [1,5,19,23,30];
    let arr2:[u8;5] = [2,4,6,8,22];

    let res: [u8;10] = merge(&arr1, &arr2);

    for val in res.iter(){
        print!("{} ", val);
    }
    println!("");

}

fn merge(ar1: &[u8;5], ar2: &[u8;5]) -> [u8;10]{

    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;
    let mut i: usize = 0;

    let mut result: [u8;10]=[0;10];

    while ptr1<5 && ptr2<5{
        if ar1[ptr1] < ar2[ptr2]{
            result[i] = ar1[ptr1];
            ptr1+=1;
        }
        else{
            result[i] = ar2[ptr2];
            ptr2+=1;
        }
        i+=1;
    }

    if ptr1>=5{
        while ptr2<5{
            result[i] = ar2[ptr2];
            ptr2+=1;
            i+=1;
        }
    }
    else if ptr2 >= 5{
        while ptr1<5{
            result[i] = ar1[ptr1];
            ptr1+=1;
            i+=1;
        }
    }
    result
}
