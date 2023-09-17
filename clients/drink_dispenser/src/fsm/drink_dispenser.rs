use crate::gpio::{GpioController, RppalGpioController};
use log::{info, warn};
use rppal::gpio::OutputPin;
use std::clone::Clone;
use strum::AsRefStr;

#[derive(Debug, thiserror::Error)]
pub enum DrinkDispenserError {
    #[error("Unexpected pin error")]
    PinError,
    #[error(transparent)]
    GpioError(#[from] rppal::gpio::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, AsRefStr)]
pub enum DrinkDispenserEvent {
    TurnOn,
    TurnOff,
    StartDrinkDispensing,
    StopDrinkDispensing,
    DrinkDispensed,
    MachineMalfunction,
    TurnOnGpioPin { gpio_pin: u8 },
    TurnOffGpioPin { gpio_pin: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, AsRefStr)]
enum OutOfOrder {
    RestockRequired,
    HardwareFault,
}

#[derive(Clone, Copy, Debug, PartialEq, AsRefStr)]
pub enum DrinkDispenserState {
    OutOfOrder(OutOfOrder),
    Idle,
    DispensingDrink,
}

#[derive(Debug, PartialEq)]
pub struct DrinkDispenserOutput;

struct GpioPinMap {
    drink_dispender: u32,
}

impl Default for GpioPinMap {
    fn default() -> Self {
        Self {
            drink_dispender: 17,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrinkDispenser<T: GpioController + Clone> {
    pub state: DrinkDispenserState,
    pub gpio_pin_map: Settings,
    gpio_controller: T,
}

impl<T: GpioController + Clone> DrinkDispenser<T> {
    pub fn new(gpio_controller: T) -> Self {
        Self {
            state: DrinkDispenserState::Idle,
            gpio_pin_map: GpioPinMap::default(),
            gpio_controller,
        }
    }

    pub fn set_state(mut self, state: DrinkDispenserState) -> Self {
        self.state = state;
        state
    }

    fn select_drink(&mut self, _drink: String) -> Result<DrinkDispenserState, DrinkDispenserError> {
        // self.gpio_controller.turn_on_pin(gpio_pinp)?;
        // self.send(DrinkDispenserEvent::DrinkSelected)?;
        Ok(())
    }

    fn start_dispensing(&mut self, gpio_pin: u8) -> Result<(), DrinkDispenserError> {
        self.gpio_controller.turn_on_pin(gpio_pin)?;
        info!("Started dispensing on GPIO pin: {}", gpio_pin);
        // self.send(DrinkDispenserEvent::StartDrinkDispensing { gpio_pin })?;
        Ok(())
    }
    fn stop_dispending(&mut self, gpio_pin: u8) -> Result<(), DrinkDispenserError> {
        self.gpio_controller.turn_off_pin(gpio_pin)?;
        info!("Stopped dispensing on GPIO pin: {}", gpio_pin);

        // self.send(DrinkDispenserEvent::StopDrinkDispensing { gpio_pin })?;
        Ok(())
    }

    pub fn handle_event(
        &mut self,
        event: DrinkDispenserEvent,
    ) -> Result<Self, DrinkDispenserError> {
        use DrinkDispenserEvent::*;
        use DrinkDispenserState::*;

        info!("Received event: {:?}", event);

        let state = match (self.state, event) {
            (_, DrinkDispenserEvent::TurnOn) => DrinkDispenserState::Idle,
            (_, DrinkDispenserEvent::TurnOff) => DrinkDispenserState::Idle,
            (Idle, StartDrinkDispensing) => {
                self.start_dispensing(gpio_pin)?;
                DrinkDispenserState::DispensingDrink
            }
            (DispensingDrink, StartDrinkDispensing) => {
                //already dispending - perhaps add to queue?
                DrinkDispenserState::DispensingDrink
            }
            (DispensingDrink, StopDrinkDispensing) => {
                //already dispending - perhaps add to queue?
                DrinkDispenserState::Idle
            }

            // Define other state transitions here

            // Fallback if no transition matches
            (current_state, _) => current_state,
        };

        Ok(state)
    }
}
