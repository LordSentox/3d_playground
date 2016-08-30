use cgmath::prelude::*;
use cgmath::{Deg, Rad, vec3, Vector3, Point3, Matrix4};
use std::sync::{Arc, Mutex};

pub trait Camera {
	fn pos(&self) -> Point3<f32>;
	fn set_pos(&mut self, pos: Point3<f32>);

	fn center(&self) -> Point3<f32>;
	fn set_center(&mut self, Point3<f32>);

	fn handle_mouse_motion(&mut self, xrel: i32, yrel: i32);

	fn view_matrix(&self) -> Matrix4<f32>;
}

pub struct FPSCamera {
	pos: Point3<f32>,
	front: Vector3<f32>,
	up: Vector3<f32>,
	yaw: Rad<f32>,
	pitch: Rad<f32>
}

impl Camera for FPSCamera {
	fn pos(&self) -> Point3<f32> {
		self.pos
	}

	fn set_pos(&mut self, pos: Point3<f32>) {
		self.pos = pos;
	}

	fn center(&self) -> Point3<f32> {
		self.pos + self.front
	}

	fn set_center(&mut self, center: Point3<f32>) {
		self.front = center - self.pos;
	}

	fn handle_mouse_motion(&mut self, xrel: i32, yrel: i32) {
		// TODO: Sensitivity should be an option, not a constant.
		let sensitivity = 0.002;
		let xoff = xrel as f32 * sensitivity;
		let yoff = yrel as f32 * sensitivity;

		self.yaw += Rad(xoff);
		self.pitch += Rad(yoff);

		// Make sure the screen doesn't get flipped by mouse-movement.
		if self.pitch > Deg(89.0).into() {
			self.pitch = Deg(89.0).into();
		}
		else if self.pitch < Deg(-89.0).into() {
			self.pitch = Deg(-89.0).into();
		}

		self.update_vectors();
	}

	fn view_matrix(&self) -> Matrix4<f32> {
		Matrix4::look_at(self.pos, self.pos + self.front, self.up)
	}
}

impl FPSCamera {
	/// Create a new camera with optional starting point. If no point is set,
	/// the camera starts at (0, 0, 0).
	pub fn new(pos: Option<Point3<f32>>) -> FPSCamera {
		let pos = match pos {
			Some(pos) => pos,
			None => Point3::new(0.0, 0.0, 0.0)
		};

		FPSCamera {
			pos: pos,
			front: vec3(0.0, 0.0, 1.0),
			up: vec3(0.0, 1.0, 0.0),
			yaw: Deg(0.0).into(),
			pitch: Deg(0.0).into()
		}
	}

	fn update_vectors(&mut self) {
		self.front.x = self.yaw.cos() * self.pitch.cos();
		self.front.y = -self.pitch.sin();
		self.front.z = self.yaw.sin() * self.pitch.cos();
		self.front = self.front.normalize();

		let right = self.front.cross(vec3(0.0, 1.0, 0.0)).normalize();
		self.up = right.cross(self.front).normalize();
	}
}
