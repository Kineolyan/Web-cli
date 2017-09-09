use std::process::Command;
use std::result::Result;
use super::util::err_to_string;

pub fn shutdown_system() -> Result<(), String> {
	println!("Shutting down...");
	match Command::new("systemctl")
		.arg("poweroff")
		.spawn() {
		Ok(_) => Ok(()),
		Err(error) => Err(err_to_string(error))
	}
}

pub fn reboot_system() -> Result<(), String> {
	println!("Rebooting...");
	match Command::new("systemctl")
		.arg("reboot")
		.spawn() {
		Ok(_) => Ok(()),
		Err(error) => Err(err_to_string(error))
	}
}