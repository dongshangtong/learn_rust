
fn createkk() {
    let mut str = String::new();
    str.push_str("hello");
    println!("str={}",str);

    let str1 = String::from("Joy Me");
    println!("str1= {}",str1);


    let str2 = "Joy me ccfdjsf".to_string();
    println!("str2 = {}",str2);
}


fn createyy() {

    let s1 = "你好啊".to_string();
    let s2 = String::from(" ,Joy me");
    let s3 = s1 + &s2;
    println!("s3={}",s3);

    let s33 = String::from("Joy");
    let s44 = String::from("Me");

    let s55 = format!("{} {}",s33,s44);

    println!("s55={}",s55);

    println!("s55.len = {}",s55.len());


}


fn main() {

   createkk();

   createyy();
    println!("Hello, world!");
}
