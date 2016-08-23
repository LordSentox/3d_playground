use gl;
use gl::types::*;
use std::ptr;
use std::ffi::CStr;
use super::{GLId, Shader};
use super::error::LinkerError as Error;

pub struct Program {
	id: GLuint,
	owned_shaders: Vec<Shader>
}

impl Program {
	/// Creates a new, empty program.
	pub fn new() -> Program {
		Program {
			id: unsafe { gl::CreateProgram() },
			owned_shaders: Vec::new()
		}
	}

	/// Attach the Shader to the program. The shader is consumed and can only be used in this program.
	/// If you want to use the shame shader for multiple programs, use the lend_shader function.
	pub fn attach_shader(&mut self, shader: Shader) {
		unsafe {
			gl::AttachShader(self.id, shader.gl_id());
			self.owned_shaders.push(shader);
		}
	}

	// TODO: Instead of having this function in the actual program, maybe it would be better to
	// put it into a Program-Builder, as to never use unlinked programs.
	pub fn link(&mut self) -> Result<(), Error> {
		unsafe {
			gl::LinkProgram(self.id);

			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut status);
			if status != (gl::TRUE as GLint) {
				let mut len = 0;
				gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
				let mut buf = Vec::with_capacity(len as usize);
				buf.set_len((len as usize) - 1);
				gl::GetProgramInfoLog(self.id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

				Err(Error::new(CStr::from_ptr(buf.as_ptr()).to_str().unwrap()))
			}
			else {
				// Deattach all owned shaders, since the program is now linked and does not require them any more.
				for shader in self.owned_shaders.drain(..) {
					gl::DetachShader(self.id, shader.gl_id());
				}

				Ok(())
			}
		}
	}

	pub fn use_program(&self) {
		unsafe { gl::UseProgram(self.id); }
	}
}

impl GLId for Program {
	fn gl_id(&self) -> GLuint {
		self.id
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.id); }
	}
}
