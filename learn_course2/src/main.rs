fn main() {

    // Rust 的基础语法、数据类型、项目结构等

    let a= 10;

    println!("自定义变量a自动推到类型:{}",a);


    let b:i32  = 20;

    println!("自定义变量n类型:{}",b);


    let mut c = 23i32;

    println!("自定义23变量为i32类型:{}",c);

    let mut d = 56_i32;

      println!("自定义56变量为i32类型:{}",d);


      let yy = add(c,d);


   println!("两个变量相加得:{}",yy);

}



fn add(i: i32, j: i32) -> i32 {
    i + j
}
