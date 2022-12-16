enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait GetDuration {
    fn get_duration(&self) -> u32;
}

impl GetDuration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 50,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 3,
        }
    }
}

#[test]
fn test_traffic_light() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    assert_eq!(red.get_duration(), 50);
    assert_eq!(green.get_duration(), 40);
    assert_eq!(yellow.get_duration(), 3);
}
