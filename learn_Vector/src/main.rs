fn main() {
    // let mut v: Vec<i32> =Vec::new();
    // for i in 0..18 {
    //     v.push(i*2);
    // }
    // println!("{:?}", v[6]);

    let v: Vec<i32> = vec![9, 9, 23, 24, 5, 3, 245, 782, 445, 5747, 3, 534];

    println!("{:?}", v[6]);

    for i in 0..v.len() {
        println!("KKKKKK{:?}", v[i]);
    }
}
