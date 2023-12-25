mod controls;
mod dbus_handler;

use std::path::Path;

use dbus_handler::create_msi_controls_handler;
use controls::power_control::Error;

const MSI_EC_MODULE_PATH: &str = "/sys/devices/platform/msi-ec/";

fn main() -> Result<(), Error> {
    let msi_ec_module = Path::new(MSI_EC_MODULE_PATH);
    if !msi_ec_module.is_dir() {
        println!("You have to install msi-ec module to use this backend. See https://github.com/BeardOverflow/msi-ec/");
        std::process::exit(1);
    }

    match create_msi_controls_handler() {
        Ok(_) => {},
        Err(e) => {
            println!("Error ocured.\n{:?}", e);
            std::process::exit(1);
        }
    };

    Ok(())
}
