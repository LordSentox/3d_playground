use cgmath::prelude::*;
use cgmath::{Rad, Vector3, Matrix4};
use std::sync::{Arc, Mutex};

pub trait Camera {
	fn fov(&self) -> Rad<f32>;
	fn set_fov<A: Into<Rad<f32>>>(&mut self, fov: A);

	fn pos(&self) -> Vector3<f32>;
	fn set_pos(&mut self, pos: Vector3<f32>);

	fn center(&self) -> Vector3<f32>;
	fn set_center(&mut self, Vector3<f32>);

	fn view_matrix(&self) -> Matrix4<f32>;
}

pub struct FPSCamera {
	pos: Arc<Mutex<Vector3<f32>>>,
	center: Vector3<f32>,
	fov: Rad<f32>
}

impl Camera for FPSCamera {
	fn fov(&self) -> Rad<f32> {
		self.fov
	}

	fn set_fov<A: Into<Rad<f32>>>(&mut self, fov: A) {
		self.fov = fov.into();
	}

	fn pos(&self) -> Vector3<f32> {
		self.pos
	}

	fn set_pos(&mut self, pos: Vector3<f32>) {
		self.pos = pos;
	}

	fn center(&self) -> Vector3<f32> {
		self.center
	}

	fn set_center(&mut self, center: Vector3<f32>) {
		self.center = center;
	}

	fn view_matrix(&self) -> Matrix4<f32> {
		Matrix4::look_at()
	}
}

impl FPSCamera {
	/// Create a new camera, that always follows the position provided.
	/// Keyboard input has to be implemented seperately, however this allows you to make the player
	/// character react to anything in the game just the way you'd like to.
	pub fn new(pos: Arc<Mutex<Vector3<f32>>>, ) -> FPSCamera {
		FPSCamera {
			pos: pos,
			center: pos.lock().unwrap() + vec3(0.0, 0.0, 1.0),
			fov: Deg(90.0).into()
		}
	}
}
