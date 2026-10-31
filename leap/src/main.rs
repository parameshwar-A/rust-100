fn main() {

    let year: i32 = 2001;

    if year % 400 == 0 || ( year % 4 ==0 && year % 100 != 0) {

        println!("{year} is a leap year");

    }
    else {
        println!("{year} is not a leap year");

    }
}

