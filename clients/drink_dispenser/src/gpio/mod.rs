use crate::fsm::drink_dispenser::{DrinkDispenser, DrinkDispenserEvent};
use rppal::gpio::{self, Gpio, OutputPin};
use std::sync::{Arc, Mutex};

pub trait GpioController {
    fn turn_on_pin(&self, pin_id: u8) -> Result<(), gpio::Error>;
    fn turn_off_pin(&self, pin_id: u8) -> Result<(), gpio::Error>;
}

#[derive(Clone)]
pub struct RppalGpioController {
    gpio: Gpio,
}

impl RppalGpioController {
    pub fn new() -> Result<Self, gpio::Error> {
        let gpio = Gpio::new()?;

        Ok(Self { gpio })
    }
}
impl GpioController for RppalGpioController {
    fn turn_on_pin(&self, pin_id: u8) -> Result<(), gpio::Error> {
        let mut pump_pin = self.gpio.get(pin_id)?.into_output();

        pump_pin.set_high();
        Ok(())
    }

    fn turn_off_pin(&self, pin_id: u8) -> Result<(), gpio::Error> {
        let mut pump_pin = self.gpio.get(pin_id)?.into_output();

        pump_pin.set_low();
        Ok(())
    }
}
