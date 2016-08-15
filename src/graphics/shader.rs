use gl;
use gl::types::*;

/// 
pub enum Error {

}

pub struct Shader {
	id: GLuint,
}

impl Shader {
	/// Compile the Shader from a str representing code in the GLSL.
	pub fn from_str(code: &str) -> Result<Shader, Error> {

	}

	pub fn from_file(name: &str) -> Result<Shader, Error> {

	}
}
