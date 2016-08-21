use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShaderError {
	CompileError(String),
	FileError(io::Error),
	FileNameError
}

impl fmt::Display for ShaderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&ShaderError::CompileError(ref err) => write!(f, "{}", err),
			&ShaderError::FileError(ref err) => write!(f, "{}", err),
			&ShaderError::FileNameError => write!(f, "Only able to load files with file-endings .vert, .frag or .geom")
		}
	}
}

impl Error for ShaderError {
	fn description(&self) -> &str { "Could not load shader." }
}

#[derive(Debug)]
pub struct LinkerError {
	error_msg: String
}

impl LinkerError {
	pub fn new(msg: &str) -> LinkerError {
		LinkerError {
			error_msg: msg.to_string()
		}
	}
}

impl fmt::Display for LinkerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.error_msg)
	}
}

impl Error for LinkerError {
	fn description(&self) -> &str { "Could not link program." }
}

#[derive(Debug)]
pub struct WindowError {
	error_msg: String
}

impl WindowError {
	pub fn new(msg: &str) -> WindowError {
		WindowError {
			error_msg: msg.to_string()
		}
	}
}

impl fmt::Display for WindowError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.error_msg)
	}
}

impl Error for WindowError {
	fn description(&self) -> &str { "Could not open window." }
}
