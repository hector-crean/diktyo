

// use rppal::gpio::{self, Gpio, OutputPin};


#[derive(thiserror::Error, Debug)]
pub enum GpioControllerError {}

pub trait GpioController: Sized {
    type T;
    fn new() -> Result<Self, GpioControllerError>;
    fn turn_on_pin(&self, pin_id: u8) -> Result<Self::T, GpioControllerError>;
    fn turn_off_pin(&self, pin_id: u8) -> Result<Self::T, GpioControllerError>;
}

pub struct MockGpioController;

impl GpioController for MockGpioController {
    type T = ();

    fn new() -> Result<Self, GpioControllerError> {
        Ok(MockGpioController)
    }
    fn turn_on_pin(&self, _pin_id: u8) -> Result<Self::T, GpioControllerError> {
        Ok(())
    }
    fn turn_off_pin(&self, _pin_id: u8) -> Result<Self::T, GpioControllerError> {
        Ok(())
    }
}

// #[derive(Clone)]
// pub struct RppalGpioController {
//     gpio: Gpio,
// }

// impl RppalGpioController {
//     pub fn new() -> Result<Self, gpio::Error> {
//         let gpio = Gpio::new()?;

//         Ok(Self { gpio })
//     }
// }
// impl GpioController for RppalGpioController {
//     type E = gpio::Error;
//     type T = ();
//     fn turn_on_pin(&self, pin_id: u8) -> Result<Self::T, Self::E> {
//         let mut pump_pin = self.gpio.get(pin_id)?.into_output();

//         pump_pin.set_high();
//         Ok(())
//     }

//     fn turn_off_pin(&self, pin_id: u8) -> Result<Self::T, Self::E> {
//         let mut pump_pin = self.gpio.get(pin_id)?.into_output();

//         pump_pin.set_low();
//         Ok(())
//     }
// }
