#![cfg_attr(not(test), no_std)]
use embedded_hal::digital::v2::OutputPin;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Motor<A: OutputPin, B: OutputPin> {
    action_pin: A,
    direction_pin: B,
}

impl<A: OutputPin, B: OutputPin> Motor<A, B> {
    pub fn new(action_pin: A, direction_pin: B) -> Self {
        Motor {
            action_pin,
            direction_pin,
        }
    }

    pub fn forward(&mut self) {
        _ = self.action_pin.set_high();
        _ = self.direction_pin.set_high();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
      FakePin {}

      impl OutputPin for FakePin {
        type Error = u32;

        // Required methods
        fn set_low(&mut self) -> Result<(), u32>;
        fn set_high(&mut self) -> Result<(), u32>;
      }
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_motor_forward() {
        // given
        let mut action_pin = MockFakePin::new();
        action_pin.expect_set_high().times(1).returning(|| Ok(()));

        let mut direction_pin = MockFakePin::new();
        direction_pin
            .expect_set_high()
            .times(1)
            .returning(|| Ok(()));

        // when
        let mut motor = Motor::new(action_pin, direction_pin);
        motor.forward();
    }
}
