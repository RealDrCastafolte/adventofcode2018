#[derive(Debug)]
pub struct Device {
    frequency: i32,
    frequency_reached_twice: Option<i32>,
    calibrations: Vec<i32>
}

impl Device {

    pub fn new() -> Device {
        Device {
            frequency: 0,
            frequency_reached_twice: None,
            calibrations: vec![0]
        }
    }

    pub fn frequency(&self) -> i32 {
        self.frequency
    }

    pub fn calibrate(&mut self, frequency: i32) {
        self.frequency = self.frequency + frequency;
        if self.frequency_reached_twice.is_none() && self.calibrations.contains(&self.frequency) {
            self.frequency_reached_twice = Some(self.frequency);
        }
        self.calibrations.push(self.frequency);
    }

    pub fn first_reached_twice(&self) -> Option<i32> {
        self.frequency_reached_twice
    }

}

#[cfg(test)]
mod tests {

    use super::Device;

    #[test]
    fn should_have_frequency_0() {
        assert_eq!(Device::new().frequency(), 0);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_3() {
        let mut device = Device::new();
        [1, -2, 3, 1].iter()
                     .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.frequency(), 3);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_0() {
        let mut device = Device::new();
        [1, 1, -2].iter()
                  .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.frequency(), 0);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_minus_6() {
        let mut device = Device::new();
        [-1, -2, -3].iter()
                    .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.frequency(), -6);
    }

    #[test]
    fn should_reach_frequency_0_twice() {
        let mut device = Device::new();
        [1, -1, 1, -1, 1, -1, 1, -1].iter()
                                    .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.first_reached_twice().unwrap(), 0);
    }

    #[test]
    fn should_reach_frequency_10_twice() {
        let mut device = Device::new();
        [3, 3, 4, -2, -4, 3, 3, 4, -2, -4, 3, 3, 4, -2, -4].iter()
                                                           .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.first_reached_twice().unwrap(), 10);
    }

    #[test]
    fn should_reach_frequency_5_twice() {
        let mut device = Device::new();
        [-6, 3, 8, 5, -6, -6, 3, 8, 5, -6, -6, 3, 8, 5, -6].iter()
                                                           .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.first_reached_twice().unwrap(), 5);
    }

    #[test]
    fn should_reach_frequency_14_twice() {
        let mut device = Device::new();
        [7, 7, -2, -7, -4, 7, 7, -2, -7, -4, 7, 7, -2, -7, -4].iter()
                                                              .for_each(|frequency| device.calibrate(*frequency));
        assert_eq!(device.first_reached_twice().unwrap(), 14);
    }
}