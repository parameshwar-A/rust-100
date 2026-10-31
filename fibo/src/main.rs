fn main() {

    let mut n0: i32 = 0;
    let mut n1: i32 = 1;


    let n: i32 = 10;
    let mut val:i32;

    let mut count: i32 = 0;

    while count <= n{

        if count == 0{
            println!("0");
        }

        else if count == 1 {
            println!("1");
        }

        else {
            val = n0 + n1;
            println!("{val}");
            n0 = n1;
            n1 = val;
        }
        count+=1
    }
    
}
