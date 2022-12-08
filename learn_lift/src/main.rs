struct StuA<'a> {
    name: &'a str,
}


impl <'b> StuA<'b> {
    fn do_something(&self) -> i32 {
        4
    }

 //   fn do_something2(&self, s:&str) -> &str{
//     等价于下面
    fn do_something2<'c>(&self, s: &'c str) -> &'c str{
        s
    }

    fn do_something3<'a>(&self, s: &'a str) -> &'a str {
        s
    }
}



fn main() {

    let s = String::from("hello");
    let a = StuA{name:&s};
    println!("{}",a.do_something());

    let s2 = String::from("wecome");
    println!("{}",a.do_something2(s2.as_str()));

    let s3 = String::from("myshehe");
    println!("{}",a.do_something3(s3.as_str()));


   // a.do_something3()
    println!("Hello, world!");
}
