use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum Messages {
//     UnlockBike { bike_id: Uuid },
//     LockBike { bike_id: Uuid },
//     SwitchBikeOnline { socket_address: SocketAddr },
//     SwitchBikeOffline { socket_address: SocketAddr },
// }

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum BibeMsg {
    UnlockBike { bike_id: Uuid },
    LockBike { bike_id: Uuid },
    SwitchBikeOnline { socket_address: SocketAddr },
    SwitchBikeOffline { socket_address: SocketAddr },
    // SwitchOnline { socket_address: SocketAddr },
    // SwitchOffline { socket_address: SocketAddr },
    // StartDrinkDispensing { dispenser_id: Uuid },
    // StopDrinkDispensing { dispenser_id: Uuid },

    // SelectDrink { drink_code: Uuid },

    // DrinkDispensed,
    // MachineMalfunction,
    // TurnOnGpioPin { gpio_pin: u8 },
    // TurnOffGpioPin { gpio_pin: u8 },
}
