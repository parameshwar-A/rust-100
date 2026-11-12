use std::collections::HashMap;
fn main() {

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    insert_data(&mut map, String::from("Engineering"), String::from("Paramesh"));
    insert_data(&mut map, String::from("Engineering"), String::from("Divya"));
    insert_data(&mut map, String::from("Mathematics"), String::from("Dinesh"));


    for (key,val) in &map{
        println!("?{} ?{:#?}", key, val);
    }
}

fn insert_data(dmap: &mut HashMap<String, Vec<String>> , key:String, value: String){

    dmap.entry(key.clone()).or_insert(Vec::new());
    if let Some(val) = dmap.get_mut(&key) {
        val.push(value);
    }

}
