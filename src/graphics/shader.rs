use gl;
use gl::types::*;
use super::error::ShaderError as Error;
use std::ptr;
use std::ffi::CString;

pub struct Shader {
	id: GLuint,
}

impl Shader {
	/// Compile the Shader from a str representing code in the GLSL.
	pub fn from_str(code: &str, ty: GLenum) -> Result<Shader, Error> {
		unsafe {
			let shader_id = gl::CreateShader(ty);

			let c_str = CString::new(code.as_bytes()).expect("Could not convert str into CString");
			gl::ShaderSource(shader_id, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(shader_id);

			// Check for errors compiling the shader
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);

			if status != (gl::TRUE as GLint) {
				let mut len = 0;
				gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1);
				gl::GetShaderInfoLog(shader_id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

				Err(Error::CompileError(String::from_utf8(buf).unwrap()))
			}
			else {
				Ok(Shader {
					id: shader_id
				})
			}
		}
	}

	/// Load and compile a shader from file.
	/// Takes the filename as parameter and decides what type the shader is
	/// depending on the file ending.
	/// .vert for Vertex-Shaders
	/// .frag for Fragment-Shaders
	/// .geom for Geometry-Shaders
	pub fn from_file(name: &str) -> Result<Shader, Error> {
		// Determine the type of shader we're dealing with.

	}
}
