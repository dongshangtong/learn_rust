



fn varbblearn() {

    let bbff = false;
    println!("bbff = {}", bbff);

    let yy = 'u';
    println!("yy = {}",yy);

    println!("usize:{}",usize::max_value());

    let arr : [u32; 6]= [8,9,0,9,4,2];

    println!("arr[0] = {}",arr[0]);


    // 元祖;
    let kkk:(u32,char) = (3,'r');

    println!("{}",kkk.0);

}

fn varlear() {
    let a = 1;
    println!("a = {}", a);

    let mut cc: u32 = 44;
    println!("cc = {}", cc);

    cc = 55;

    println!("cc = {}",cc);

    // 前面已经定义变量cc , 在定义cc ,还是可以定于， 这个rust隐藏性
    let cc: f32 = 3.6;
    println!("cc == {}",cc);


    println!("Hello, world!");

}

fn three(n: i32) -> i32 {
    n + 3
}

fn yyyy(a: i32 ,b: i32) -> i32 {

   let res = a + b ;

   return res;
}

fn my_fun3(){
    let mut i = 0 ;
    while i  != 13 {
        i += 1;
    }
    println! ("my_kkk == {}",i);

}


fn my_fun4() {
    let arr333 : [i32; 8] = [1,2,3,4,5,6,7,8];

    for el in arr333.iter() {
        println!("le ={}",el);
    }

}

fn  my_fun5() {
    let mut yyy = 0;
    loop {
        
        println!("yyy = {}",yyy);
        yyy += 1;

        if yyy == 20 {
            break;
        }

    }
}

fn my_fun6() {

    let s = String::from("wecome");

    println!("s = {}", &s[..3]);

}


fn main() {

 //  varlear();

 // varbblearn();

 let c : i32 = 4 ;
 let v : i32 = 8 ;




 let rr = yyyy(c,v);

 println!("rr = {}",rr);


 let y :i32 =  {
     let k :i32 = 9 ;
     k + 4
 };

 println!("kk = {}",y);
let  conn = true;

let gg =  if  conn  {
    5
}else {
    9
};


println!("x== {}",gg);


// my_fun3();

// my_fun4();

 // my_fun5();

my_fun6();

}

