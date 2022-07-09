// 04 交通信号灯trait，返回不同的灯持续的时间
pub trait GetLightTime {
    fn get_light_time(&self) -> u32;
}

// 交通信号灯枚举
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl std::fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sig = match self {
            Self::Red => "Red",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
        };
        write!(f, "{}", sig)
    }
}

impl std::str::FromStr for TrafficLight {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sig = s.to_lowercase();
        match sig.as_str() {
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            _ => Err(format!("{} is not a valid traffic light color", sig)),
        }
    }
}

// 获取不同灯的持续时间
impl GetLightTime for TrafficLight {
    fn get_light_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 30,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_time() {
        assert_eq!(TrafficLight::Red.get_light_time(), 20);
    }

    #[test]
    fn test_green_time() {
        assert_eq!(TrafficLight::Green.get_light_time(), 30);
    }

    #[test]
    fn yellow_time_is_too_short() {
        assert!(TrafficLight::Yellow.get_light_time() > 1);
    }
}
