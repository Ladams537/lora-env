#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub gas_resistance: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceCommand {
    Sleep,
    Reset,
    SetInterval(u32),
}
