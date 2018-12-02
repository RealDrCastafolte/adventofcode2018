#[derive(Debug)]
pub struct Device {
    frequency: i32
}

impl Device {

    pub fn new(frequency: i32) -> Device {
        Device {
            frequency: frequency
        }
    }

    pub fn frequency(&self) -> i32 {
        self.frequency
    }

    pub fn calibrate(&self, frequency: i32) -> Device {
        Device {
            frequency: self.frequency + frequency
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Device;

    #[test]
    fn should_have_frequency_0() {
        assert_eq!(Device::new(0).frequency(), 0);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_3() {
        let device = Device::new(0).calibrate(1)
                                   .calibrate(-2)
                                   .calibrate(3)
                                   .calibrate(1);
        assert_eq!(device.frequency(), 3);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_0() {
        let device = Device::new(0).calibrate(1)
                                   .calibrate(1)
                                   .calibrate(-2);
        assert_eq!(device.frequency(), 0);
    }

    #[test]
    fn when_calibrate_then_should_have_frequency_minus_6() {
        let device = Device::new(0).calibrate(-1)
                                   .calibrate(-2)
                                   .calibrate(-3);
        assert_eq!(device.frequency(), -6);
    }
}