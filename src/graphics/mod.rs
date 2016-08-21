pub mod error;
pub use self::error::*;

pub mod shader;
pub use self::shader::*;

pub mod program;
pub use self::program::*;

pub mod window;
pub use self::window::*;

use gl::types::GLuint;
/// Something representing a GL-Object always has an internal id by which it can be
/// identified. Therefore, their Rust-counterparts have to implement this trait.
pub trait GLId {
	/// Returns the id of the internal GL-Object.
	fn gl_id(&self) -> GLuint;
}
