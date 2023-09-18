use crate::gpio::{GpioController, GpioControllerError, MockGpioController};
// use rppal::gpio::OutputPin;
use std::clone::Clone;

#[derive(Debug, thiserror::Error)]
pub enum DrinkDispenserError {
    #[error("Unexpected pin error")]
    PinError,
    #[error(transparent)]
    GpioError(#[from] GpioControllerError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutOfOrder {
    RestockRequired,
    HardwareFault,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrinkDispenserState {
    OutOfOrder(OutOfOrder),
    Idle,
    DispensingDrink,
}

#[derive(Debug, PartialEq)]
pub struct DrinkDispenserOutput;

#[derive(Debug, Clone, Copy)]
pub struct GpioPinMap {
    drink_dispenser: u8,
}

impl Default for GpioPinMap {
    fn default() -> Self {
        Self {
            drink_dispenser: 17,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrinkDispenser<T: GpioController + Clone> {
    pub state: DrinkDispenserState,
    pub gpio_pin_map: GpioPinMap,
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
        self
    }

    fn start_dispensing(&mut self, gpio_pin: u8) -> Result<(), DrinkDispenserError> {
        self.gpio_controller.turn_on_pin(gpio_pin)?;
        // self.send(DrinkDispenserEvent::StartDrinkDispensing { gpio_pin })?;
        Ok(())
    }
    fn stop_dispending(&mut self, gpio_pin: u8) -> Result<(), DrinkDispenserError> {
        self.gpio_controller.turn_off_pin(gpio_pin)?;

        // self.send(DrinkDispenserEvent::StopDrinkDispensing { gpio_pin })?;
        Ok(())
    }

    pub fn handle_event(
        &mut self,
        event: DrinkDispenserEvent,
    ) -> Result<DrinkDispenserState, DrinkDispenserError> {
        use DrinkDispenserEvent::*;
        use DrinkDispenserState::*;

        let state = match (self.state, event) {
            (_, DrinkDispenserEvent::TurnOn) => DrinkDispenserState::Idle,
            (_, DrinkDispenserEvent::TurnOff) => DrinkDispenserState::Idle,
            (Idle, StartDrinkDispensing) => {
                self.start_dispensing(self.gpio_pin_map.drink_dispenser)?;
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
