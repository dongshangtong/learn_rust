
use std::collections::HashMap;

fn main() {

    let mut scores: HashMap<String,i32> = HashMap::new();


    scores.insert(String::from("blue"),20);
    scores.insert(String::from("Red"),50);

    let keys = vec![String::from("blue"),String::from("Red")];
    let values = vec![10,90];
    let scores1:HashMap<_,_> = keys.iter().zip(values.iter()).collect();


    let k = String::from("blue");

    if let Some(v) = scores.get(&k){
        println!("v = {}",v);
    };

    let k2 = String::from("Redkk");
    let v2 = scores1.get(&k2);
    match v2 {
        Some(va) => println!("v2={}",va),
        None => println!("None"),
    }


    for (k ,v) in &scores {
        println!("{}kk={}",k,v);
    }






    // println!("map={}",scores);
    println!("Hello, world!");
}
