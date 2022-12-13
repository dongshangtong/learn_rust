
enum  TrafficLight {
    Red,
    Green,
    Yellow,
}


impl TrafficLight {
    fn time(&self) -> u8 {
        60
    }
}



// enum IPAddr {
//     ipv4(string),
//     ipv6(string),
// }


// enum Option<T> {
//     Some(T),
//     None(T),
// }



fn main() {

    // let yelllow = TrafficLight::Yellow;
    // println!("kkkkkkkkkk:{}",yelllow.time());
    let five = Some(5);

    let six = five.map(|i| i+ 1);

    println!("six:{:?}",six);


}
