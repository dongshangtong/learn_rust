

fn main() {

    // 闭包 定义==》|参数,参数| -> i32 {a + r};
    let myclosure = |s,a| -> i32 {s + a};


    println!("myclosure={}",myclosure(-3,9));


    println!("Hello, world!");
}
