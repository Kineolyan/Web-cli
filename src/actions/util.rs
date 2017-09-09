use std::io::Error;

pub fn err_to_string(e: Error) -> String {
	String::from(
			match e.get_ref() {
				None => "Some error",
				Some(err) => err.description()
			})
}