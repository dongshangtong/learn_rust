
use std::mem::size_of_val;

fn main() {

    // test_coures1()
    // test_coures2()
    // test_coures3()

    test_coures4();
}


fn test_coures4() {
    let unit:() = ();
    println!("kkkk:{}",size_of_val(&unit));

}


fn test_coures3() {

    let _f: bool = false;

    let t = false;
    if !t {
        println!("hello ,world");
    }

}


fn test_coures2() {

    let c1 = '中';

    print_char(c1);
}

fn print_char(c : char) {
    println!("{}",c);
}


fn test_coures1(){

    let c1 = 'a';

     println!("KKKK1K:{}",size_of_val(&c1));

    let c2 = '中';
     println!("KKKK2K:{}",size_of_val(&c2));
}