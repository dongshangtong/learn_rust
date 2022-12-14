use std::collections::HashMap;
use std::time::{systime};



fn main() {
   

    let mut transcript:HashMap<&str, u32> = HashMap::new();


    transcript.insert("alice",89);
    transcript.insert("bob",78);
    transcript.insert("yid", 82);

    match transcript.get("alice") {

            Some(data) => println!("alice {:?}",data),
            None => println!("alice not found"),

    }
    

    transcript.remove("alice");


    match transcript.get("alice") {

            Some(data) => println!("alice {:?}",data),
            None => println!("alice not found"),

    }

    let now = systime::new();
    let timestamp = now.duration_since(systime::UNIX_EPOCH);

}
