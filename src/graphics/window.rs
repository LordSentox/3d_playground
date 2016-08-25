use gl;
use sdl2;
// use super::error::WindowError as Error;
use sdl2::video::{GLProfile, WindowBuildError as BuildError};
use sdl2::event::Event;

use std::mem;

pub struct Window {
	sdl_context: sdl2::Sdl,
	sdl_window: sdl2::video::Window,
	gl_context: sdl2::video::GLContext,
	event_pump: sdl2::EventPump
}

impl Window {
	/// Create a new window. The window size is optional. If the size is not provided, the
	/// window will be created at desktop resolution.
	pub fn new(title: &str, fullscreen: bool, size: Option<(u32, u32)>) -> Result<Window, BuildError> {
		let sdl = match sdl2::init() {
			Ok(sdl) => sdl,
			Err(err) => return Err(BuildError::SdlError(err))
		};
		let video = match sdl.video() {
			Ok(video) => video,
			Err(err) => return Err(BuildError::SdlError(err))
		};

		// This library uses OpenGL version 330 core by default.
		video.gl_attr().set_context_profile(GLProfile::Core);
		video.gl_attr().set_context_version(3, 3);

		// TODO: This is not a very nice way of doing it. However I couldn't find any corresponding
		// sdl2-crate functions that would allow me to do this in a straightforward manner.
		let window = match (size, fullscreen) {
			(None, false) => video.window(&title, 800, 600).opengl().build(),
			(None, true) => video.window(&title, 800, 600).opengl().fullscreen_desktop().build(),
			(Some((w, h)), false) => video.window(&title, w, h).opengl().build(),
			(Some((w, h)), true) => video.window(&title, w, h).opengl().fullscreen().build()
		};

		// Try to unwrap the result built in the previous step.
		let window = match window {
			Ok(window) => window,
			Err(err) => return Err(err)
		};

		let gl_context = match window.gl_create_context() {
			Ok(context) => {
				// Load gl functions and return the resulting context.
				gl::load_with(|s| unsafe {
                    mem::transmute(video.gl_get_proc_address(s))
                });

				context
			}
			Err(err) => return Err(BuildError::SdlError(err))
		};

		// Make the window current, to register as the correct GL-context.
		window.gl_make_current(&gl_context);

		let event_pump = match sdl.event_pump() {
			Ok(pump) => pump,
			Err(err) => return Err(BuildError::SdlError(err))
		};

		Ok(Window {
			sdl_context: sdl,
			sdl_window: window,
			gl_context: gl_context,
			event_pump: event_pump
		})
	}

	pub fn poll_event(&mut self) -> Option<Event> {
		self.event_pump.poll_event()
	}

	/// Clear the window. Resets colour buffer, depth buffer and stencil buffer.
	pub fn clear(&self) {
		unsafe {
			gl::ClearColor(0.0, 1.0, 0.5, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
		}
	}

	/// Swap the framebuffers to show the new rendered scene.
	pub fn display(&self) {
		self.sdl_window.gl_swap_window();
	}
}
