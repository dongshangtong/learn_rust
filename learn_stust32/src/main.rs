
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}


#[derive(Debug)]
struct Point2<T,U> {
    a:T,
    h:U,
}

fn main() {

    let integetr = Point{
        x:7,
        y:20,
    };

    println!("{:#?}",integetr);

    let float = Point{x:1.2,y:5.7};

    println!("{:?}",float);


    let yys = Point2{a:90,h:'m'};

    println!("{:#?}",yys);


    println!("Hello, world!");
    userens();
}


enum Option<T> {
    Some(T),
    None,
}
enum Result<T,E> {
    OK(T),
    Err(E)
}


fn userens() {

let cat = Cat {a:5,b:9};
println!("a={}",cat.get_a());
println!("b={}",cat.get_b());

}

struct Cat<T> {
    a:T,
    b:T,
}

impl<T> Cat<T> {

    fn get_a(&self) -> &T{
        &self.a
    }


    fn get_b(&self) ->&T {
        &self.b
    }
    
}