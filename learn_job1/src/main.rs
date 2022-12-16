trait SignalLight {
    fn duration(&self) -> u32;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl SignalLight for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());
}
