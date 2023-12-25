use dbus::blocking::Connection;
use dbus_crossroads::{
    Crossroads,
    MethodErr
};
use thiserror::Error;

use crate::controls::{
    power_control,
    power_control::{
        get_available_power_modes,
        get_current_power_mode,
        set_power_mode,
        PowerMode
    }
};

const BUS_NAME: &str = "org.msi_ec_dbus";

#[derive(Debug, Error)]
pub enum ControlsHandlerError {
    #[error(transparent)]
    PowerControlError(#[from] power_control::Error),
    
    #[error(transparent)]
    DBusError(#[from] dbus::Error)
}

impl From<power_control::Error> for MethodErr {
    fn from(value: power_control::Error) -> Self {
        MethodErr::failed(&value)
    }
}

pub fn create_msi_controls_handler() -> Result<(), ControlsHandlerError>{
    let conn = Connection::new_system()?;
    conn.request_name(BUS_NAME, false, true, false)?;
    let mut cr = Crossroads::new();

    let available_power_modes = get_available_power_modes()?;
    let available_power_modes_str = available_power_modes.iter()
    .map(|&mode| String::try_from(mode).unwrap())
    .collect::<Vec<String>>()
    .join("\n");

    println!("DEBUG");

    let get_power_modes_iface = cr.register(BUS_NAME, |b| {
        b.method("GetAvailablePowerModes", (), ("reply",), move |_, _, (): ()| {
            println!("DEBUG");
            Ok((format!("{}", available_power_modes_str),))
        });
        b.method("GetCurrentPowerMode", (), ("reply",), |_, _, (): ()| {
            let current_mode_str = String::from(get_current_power_mode()?);
            Ok((format!("{}", current_mode_str),))
        });
        b.method("SetPowerMode", ("power_mode",), ("reply",), move |_, _, (power_mode,): (String,)| {
            let target_power_mode = PowerMode::from(power_mode);
            if !available_power_modes.contains(&target_power_mode) {
                return Err(MethodErr::invalid_arg("This power mode is not available."))
            }
            match set_power_mode(target_power_mode) {
                Ok(_) => Ok((format!("Success"),)),
                Err(e) => Err(e.into())
            }
            
        });
    });

    cr.insert("/power_control", &[get_power_modes_iface], ());
    cr.serve(&conn)?;
    Ok(())
}
