use sdl2;
// use super::error::WindowError as Error;
use sdl2::video::WindowBuildError as BuildError;

pub struct Window {
	sdl_context: sdl2::Sdl,
	sdl_window: sdl2::video::Window,
	gl_context: sdl2::video::GLContext
}

impl Window {
	/// Create a new window. The window size is optional. If the size is not provided, the
	/// window will be created at desktop resolution.
	pub fn new(title: &str, fullscreen: bool, size: Option<(u32, u32)>) -> Result<Window, WindowBuildError> {
		let sdl = match sdl2::init() {
			Ok(sdl) => sdl,
			Err(err) => return Err(BuildError::SdlError(err))
		};
		let video = match sdl.video() {
			Ok(video) => video,
			Err(err) => return Err(BuildError::SdlError(err))
		};

		// TODO: This is not a very nice way of doing it. However I couldn't find any corresponding
		// sdl2-crate functions that would allow me to do this in a straightforward manner.
		let window = match (size, fullscreen) {
			(None, false) => video.window(&title, 800, 600).opengl().build(),
			(None, true) => video.window(&title, 800, 600).opengl().fullscreen_desktop().build(),
			(Some((w, h)), false) => video.window(&title, w, h).opengl().build(),
			(Some((w, h)), true) => video.window(&title, w, h).opengl().fullscreen().build()
		}

		// Try to unwrap the result built in the previous step.
		let window = match window {
			Ok(window) => window,
			Err(err) => return Err(err)
		};

		let gl_context = match window.gl_create_context() {
			Ok(context) => context,
			Err(err) => return Err(BuildError::SdlError(err))
		};

		Window {
			sdl_context: sdl,
			sdl_window: window,
			gl_context: gl_context
		}
	}
}
