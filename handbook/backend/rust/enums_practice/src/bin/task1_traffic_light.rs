/// Task 1: Traffic light — duration per color (seconds).

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Duration: {} sec", light.duration());

    for light in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        println!("{:?} -> {} sec", light, light.duration());
    }
}
