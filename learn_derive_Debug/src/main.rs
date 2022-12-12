

fn main() {
    // let mut point = Point{
    //    x: 12,y:12,
        
    // };

    // println!("KKKKK{}",point.x == point.y);

    // 实现 这个 Default
    let p = Point::default();

    println!("{:?}",p)



}


#[derive(Debug,PartialEq,Default)]
struct Point {
    x: i32,
    y: i32,
}

