extern crate cgmath;
extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use gl::types::*;
use std::mem;
use std::ptr;

use cgmath::*;

pub mod graphics;
use graphics::*;

// Vertex data
static VERTEX_DATA: [GLfloat; 18] = [
	// Positions         // Colors
	 0.5, -0.5, -1.0,  1.0, 0.0, 0.0,   // Bottom Right
	-0.5, -0.5, 0.5,  0.0, 1.0, 0.0,   // Bottom Left
	 0.0,  0.5, 2.0,  0.0, 0.0, 1.0    // Top
];

fn main() {
	let camera = FPSCamera::new(Some(Point3::new(0.0, 0.0, -3.0)));
	let mut window = Window::new("McSwag Swag", true, None, Box::new(camera)).unwrap();

	// Load test shaders to draw the triangle.
	let vert = Shader::from_file("data/shaders/test.vert").unwrap();
	let frag = Shader::from_file("data/shaders/test.frag").unwrap();
	let mut program = Program::new();
	program.attach_shader(vert);
	program.attach_shader(frag);
	program.link().unwrap();

	let mut vao = 0;
	let mut vbo = 0;

	unsafe {
	// Create Vertex Array Object
	gl::GenVertexArrays(1, &mut vao);
	gl::GenBuffers(1, &mut vbo);

	gl::BindVertexArray(vao);

	// Create a Vertex Buffer Object and copy the vertex data to it
	gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	gl::BufferData(gl::ARRAY_BUFFER,
				(VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				mem::transmute(&VERTEX_DATA[0]),
				gl::STATIC_DRAW);

	// Position
	gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as i32, ptr::null());
	gl::EnableVertexAttribArray(0);

	// Colour
	gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as i32, (3 * mem::size_of::<GLfloat>()) as *const _);
	gl::EnableVertexAttribArray(1);
	}

	let mut event_pump = window.event_pump();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
					break 'running
				},
				Event::MouseMotion {xrel, yrel, ..} => {
					// Let the camera handle the event.
					window.camera_mut().handle_mouse_motion(xrel, yrel);
				}
				_ => {}
			}
		}

		// Character camera helper vectors.
		let front = window.camera().front();
		let right = front.cross(vec3(0.0, 1.0, 0.0));

		// Ignore y values.
		let front = vec2(front.x, front.z).normalize();
		let right = vec2(right.x, right.z).normalize();

		// Handle character movement.
		let key_state = event_pump.keyboard_state();
		let mut dir: Vector2<f32> = vec2(0.0, 0.0);
		if (key_state.is_scancode_pressed(Scancode::Comma)) {
			dir += front;
		}
		if (key_state.is_scancode_pressed(Scancode::O)) {
			dir -= front;
		}
		if (key_state.is_scancode_pressed(Scancode::E)) {
			dir += right;
		}
		if (key_state.is_scancode_pressed(Scancode::A)) {
			dir -= right;
		}

		window.clear();

		window.use_program(&program);

		unsafe {
			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
	 	}

		window.display();
	}
}
