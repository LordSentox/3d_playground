use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShaderError {
	CompileError(String),
	FileError(io::Error)
}

impl fmt::Display for ShaderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&ShaderError::CompileError(ref err) => write!(f, "{}", err),
			&ShaderError::FileError(ref err) => write!(f, "{}", err)
		}
	}
}

impl Error for ShaderError {
	fn description(&self) -> &str { "Could not load shader." }
}
