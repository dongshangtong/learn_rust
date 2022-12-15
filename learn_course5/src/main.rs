fn main() {

// test_coures1();
// test_coures2();
test_coures3();
}

fn test_coures1(){

let v = {
    let mut x = 1;
    x+ 2
};

// assert_eq!(v,())

println!("KKKK:{}", v);

}


fn test_coures2(){

    let v = {
        let mut x = 1;
        x += 2;
        x
    };
assert_eq!(v,3);

    println!("K2KKK:{}", v);
}


fn test_coures3() {
    let v = {
        let x = 3;
        x
    };
    assert!(v == 3);

     println!("K3KKK:{}", v);
}




