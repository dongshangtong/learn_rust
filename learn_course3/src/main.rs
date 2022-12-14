fn main() {
    // let x = 5;
    // let mut x = 5;

    // println!(" the value of x is: {} ", x);

    // x = 8;
    // println!("he value of x is: {}", x);

    // let bb = 10;

    // // 没有使用的，使用在前加下划线，就没有警告。
    // let _gg = 45;

    // let (a, b, c, d, e);

    // (a, b) = (1, 2);

    // [c, .., d, e] = [9, 2, 3, 4, 5];

    // println!("KKKKK:{}", a);

    // // 定义常量
    // const MAX_POINTS: u32 = 100_000;

    // let mut spaces = "   ";
    // let kdf = spaces.len();

    // println!("KKKddKK:{}", kdf);

    // test_coures1();

    // test_coures2();

    // test_coures3();

    // test_coures4();

    //  test_coures5();

    // test_coures6();
    // test_coures7();

    // test_coures8();

    test_coures9();
}

fn test_coures1() {
    let x: i32 = 5;

    let y: i32;

    println!("{} is equal to 5", x);
}

fn test_coures2() {
    let mut x = 1;
    x += 2;
    println!("{} is equal to 3", x);
}

fn test_coures3() {
    //

    let x: i32 = 11;
    let y: i32 = 45;

    {
        let y: i32 = 90;

        println!(" The value of x is {} and value of  y is {} ", x, y);
    }

    println!(" The value of x is {} and value of  y is {} ", x, y);
}

fn test_coures4() {
    let x = test_coures4_1();

    println!("{}, world ", x);
}

fn test_coures4_1() -> String {
    let x = "hello".to_string();

    x
}

fn test_coures5() {
    let x = test_coures5_1();
    println!("{}, world", x);
}

fn test_coures5_1() -> &'static str {
    let x = "hello";

    x
}

fn test_coures6() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x);
}

fn test_coures7() {
    let mut x: i32 = 1;
    x = 7;
    let mut x = x;

    x += 3;

    let y = 4;
    let y = "I can alse be bound to text!";

    println!("Success!{}", x);
}

fn test_coures8() {
    let (mut x, y) = (1, 2);

    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("KxKKK:{}", x);
    println!("KyKKK:{}", y);
}

fn test_coures9() {
    let (x, y) = (1, 2);

    let x = 3;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("KxKKK:{}", x);
    println!("KyKKK:{}", y);
}
