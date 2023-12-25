use std::{
    fs::File,
    io::prelude::*
};

use thiserror::Error;

const ECO_MODE_VALUE: &str = "eco";
const COMFORT_MODE_VALUE: &str = "comfort";
const SPORT_MODE_VALUE: &str = "sport";
const TURBO_MODE_VALUE: &str = "turbo";

const AVAILABLE_MODES_PATH: &str = "/sys/devices/platform/msi-ec/available_shift_modes";
const CURRENT_MODE_PATH: &str = "/sys/devices/platform/msi-ec/shift_mode";


#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Unknown power mode can't be used!")]
    SetUnknownModeError,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Eco,
    Comfort,
    Sport,
    Turbo,
    Unknown
}

impl From<String> for PowerMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            ECO_MODE_VALUE => PowerMode::Eco,
            COMFORT_MODE_VALUE => PowerMode::Comfort,
            SPORT_MODE_VALUE => PowerMode::Sport,
            TURBO_MODE_VALUE => PowerMode::Turbo,
            _ => PowerMode::Unknown
        }
    }
}

impl From<PowerMode> for String {
    fn from(value: PowerMode) -> Self {
        match value {
            PowerMode::Eco => ECO_MODE_VALUE.to_string(),
            PowerMode::Comfort => COMFORT_MODE_VALUE.to_string(),
            PowerMode::Sport => SPORT_MODE_VALUE.to_string(),
            PowerMode::Turbo => TURBO_MODE_VALUE.to_string(),
            PowerMode::Unknown => "unknown".to_string(),
        }
    }
}


pub fn get_available_power_modes() -> Result<Vec<PowerMode>, Error> {
    let mut available_modes_file = File::open(AVAILABLE_MODES_PATH)?;
    let mut available_modes_str = String::new();
    available_modes_file.read_to_string(&mut available_modes_str)?;

    let modes: Vec<PowerMode> = available_modes_str.trim().split('\n')
        .map(|mode_str| PowerMode::from(mode_str.trim().to_string()))
        .collect();

    if modes.len() == 1 && modes.contains(&PowerMode::Unknown) {
        return Ok(Vec::new())
    }

    Ok(modes)
}


pub fn get_current_power_mode() -> Result<PowerMode, Error> {
    let mut current_mode_file = File::open(CURRENT_MODE_PATH)?;
    let mut current_mode_str = String::new();
    current_mode_file.read_to_string(&mut current_mode_str)?;
    let current_mode = PowerMode::from(current_mode_str.trim().to_string());
    Ok(current_mode)
}


pub fn set_power_mode(power_mode: PowerMode) -> Result<(), Error> {
    if power_mode == PowerMode::Unknown {
        return Err(Error::SetUnknownModeError)
    }

    let power_mode_str = String::from(power_mode);
    let mut current_mode_file = File::create(CURRENT_MODE_PATH)?;
    current_mode_file.write(power_mode_str.as_bytes())?;
    
    Ok(())
}
