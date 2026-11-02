fn old_main() {
    let s1: String = String::from(" welcome home myboys good this");
    trim_whitespace(&s1);
}


fn trim_whitespace(s01: &String) {

    let bytes = s01.as_bytes();
    let mut sind: i32 = -1;
    let mut eind: i32 = -1;

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' '{

            if sind == -1{
                sind = i as i32;
                sind = sind + 1;
                continue
            }

            if sind != -1{
                eind = i as i32;
            }

            if sind != -1 && eind != -1 {
                let s = sind as usize;
                let e = eind as usize;
                println!("{}", &s01[s..e]);
                sind = eind+1;
                eind = -1;
            }
            
        }
    }
    if sind != -1 && eind == -1{
        let s: usize = sind as usize;
        println!("{}", &s01[s..]);
    }
}
