use gl::types::*;
use super::Shader;

pub struct Program {
	id: GLuint,
	shaders: Vec<Shader>
}

impl Program {
	/// Creates a new, empty program.
	pub fn new() -> Program {
		Program {
			id: gl::CreateProgram(),
			shaders: Vec<Shader>
		}
	}

	/// Attach the Shader to the program. The shader is consumed and can only be used in this program.
	/// If you want to use the shame shader for multiple programs, use the lend_shader function.
	pub fn attach_shader(&self, shader: Shader) {
		gl::AttachShader(self.id, shader.shader_id());
	}
}
