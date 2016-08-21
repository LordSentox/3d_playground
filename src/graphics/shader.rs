use gl;
use gl::types::*;
use super::GLId;
use super::error::ShaderError as Error;
use std::ptr;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;

pub struct Shader {
	id: GLuint,
	ty: GLenum
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
					id: shader_id,
					ty: ty
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
		let ty: GLenum = if name.ends_with(".vert") {
			gl::VERTEX_SHADER
		}
		else if name.ends_with(".frag") {
			gl::FRAGMENT_SHADER
		}
		else if name.ends_with(".geom") {
			gl::GEOMETRY_SHADER
		}
		else {
			return Err(Error::FileNameError);
		};

		// Read the complete file into memory and compile the string afterwards.
		let mut file = match File::open(name) {
			Ok(file) => file,
			Err(err) => return Err(Error::FileError(err))
		};

		let mut code = String::new();
		match file.read_to_string(&mut code) {
			Ok(length) => println!("Read {}Bytes from shader file {}", length, &name),
			Err(err) => return Err(Error::FileError(err))
		}

		// Try compiling the code.
		Shader::from_str(&code, ty)
	}

	/// Get the shader type, i.e. wether this is a vertex, fragment or geometry shader.
	pub fn shader_type(&self) -> GLenum {
		self.ty
	}
}

impl GLId for Shader {
	fn gl_id(&self) -> GLuint {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id); }
	}
}
