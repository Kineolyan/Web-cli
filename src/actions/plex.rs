use std::process::Command;
use std::result::Result;
use super::util::err_to_string;

pub fn restart() -> Result<(), String> {
	println!("Restarting plex server ...");
	match Command::new("systemctl")
		.arg("restart")
		.arg("plexmediaserver.service")
		.status() {
		Ok(status) => if status.success() {
			Ok(())
		} else {
			Err(match status.code() {
				Some(code) => format!("Exited with status code: {}", code),
				None       => String::from("Process terminated by signal")
			})
		},
		Err(error) => Err(err_to_string(error))
	}
}