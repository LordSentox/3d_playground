use gl;
use sdl2;
// use super::error::WindowError as Error;
use sdl2::video::{GLProfile, WindowBuildError as BuildError};
use sdl2::EventPump;
use cgmath;
use cgmath::prelude::*;
use cgmath::{Deg, Rad, Matrix4};

use std::mem;
use std::ffi::CString;

use super::{Camera, GLId, Program};

pub struct Window {
	sdl_context: sdl2::Sdl,
	sdl_window: sdl2::video::Window,
	gl_context: sdl2::video::GLContext,
	camera: Box<Camera>,
	fov: Rad<f32>,
	proj_mat: Matrix4<f32>
}

impl Window {
	/// Create a new window. The window size is optional. If the size is not provided, the
	/// window will be created at desktop resolution.
	pub fn new(title: &str, fullscreen: bool, size: Option<(u32, u32)>, camera: Box<Camera>) -> Result<Window, BuildError> {
		let sdl = match sdl2::init() {
			Ok(sdl) => sdl,
			Err(err) => return Err(BuildError::SdlError(err))
		};

		// Turn on relative mouse movement.
		// TODO: You have to be able to toggle this to swith between FPS content
		// and 2D or other content.
		sdl.mouse().set_relative_mouse_mode(true);

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

		unsafe { gl::Enable(gl::DEPTH_TEST); }

		// Make the window current, to register as the correct GL-context.
		let _ = window.gl_make_current(&gl_context);

		let mut this = Window {
			sdl_context: sdl,
			sdl_window: window,
			gl_context: gl_context,
			camera: camera,
			fov: Rad(0.0),
			proj_mat: Matrix4::zero()
		};

		this.set_fov(Deg(45.0));

		Ok(this)
	}

	pub fn fov(&self) -> Rad<f32> {
		self.fov
	}

	pub fn set_fov<A: Into<Rad<f32>>>(&mut self, fov: A) {
		self.fov = fov.into();

		let display_mode = self.sdl_window.display_mode().expect("Could not review display mode.");

		self.proj_mat = cgmath::perspective(self.fov, display_mode.w as f32 / display_mode.h as f32, 0.1, 100.0);
	}

	pub fn event_pump(&mut self) -> EventPump {
		self.sdl_context.event_pump().expect("Could not create SDL event pump. Aborting.")
	}

	pub fn camera(&self) -> &Camera {
		&*self.camera
	}

	pub fn camera_mut(&mut self) -> &mut Camera {
		&mut *self.camera
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

	pub fn use_program(&self, program: &Program) {
		unsafe {
			// Set program as the active program.
			gl::UseProgram(program.gl_id());

			// Provide the projection and view matrix to the program.
			// TODO: Since the positions don't change for the program, these
			// should be set when creating the program, and not every frame
			// or maybe every other frame.
			let proj_loc = gl::GetUniformLocation(program.gl_id(), CString::new("proj_mat").unwrap().as_ptr());
			let view_loc = gl::GetUniformLocation(program.gl_id(), CString::new("view_mat").unwrap().as_ptr());

			// Set view and projection matrix.
			gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, self.proj_mat.as_ptr());
			gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, self.camera.view_matrix().as_ptr());
		}
	}
}
