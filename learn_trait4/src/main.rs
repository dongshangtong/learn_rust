struct Point<T>{
    x: T,
    y: T,
}

impl <T:Clone + std::cmp::PartialOrd> Point<T> {

    fn largest(&self) -> T {

        if self.x > self.y {
            self.x.clone()
        }else {
            self.y.clone()
        }

    }

}


impl Point<f32>{
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}




fn main() {
   let point = Point{
        x: 2.2,
        y:3.1,
   };



   println!("kkkkkkk:{:?}",point.largest());

    println!("kk2kkkkk:{:?}",point.distance_from_origin());


}
