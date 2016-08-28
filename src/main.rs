extern crate cgmath;
extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use cgmath::prelude::*;

pub mod graphics;
use graphics::*;

// Vertex data
static VERTEX_DATA: [GLfloat; 18] = [
	// Positions         // Colors
	 0.5, -0.5, -1.0,  1.0, 0.0, 0.0,   // Bottom Right
	-0.5, -0.5, -2.5,  0.0, 1.0, 0.0,   // Bottom Left
	 0.0,  0.5, -0.2,  0.0, 0.0, 1.0    // Top
];

fn main() {
	let mut window = Window::new("McSwag Swag", false, None).unwrap();

	// Load test shaders to draw the triangle.
	let vert = Shader::from_file("data/shaders/test.vert").unwrap();
	let frag = Shader::from_file("data/shaders/test.frag").unwrap();
	let mut program = Program::new();
	program.attach_shader(vert);
	program.attach_shader(frag);
	program.link().unwrap();

	let mut vao = 0;
	let mut vbo = 0;

	// GLuint VBO, VAO;
    // glGenVertexArrays(1, &VAO);
    // glGenBuffers(1, &VBO);
    // // Bind the Vertex Array Object first, then bind and set vertex buffer(s) and attribute pointer(s).
    // glBindVertexArray(VAO);
	//
    // glBindBuffer(GL_ARRAY_BUFFER, VBO);
    // glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
	//
    // glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(GLfloat), (GLvoid*)0);
    // glEnableVertexAttribArray(0);
	//
    // glBindBuffer(GL_ARRAY_BUFFER, 0); // Note that this is allowed, the call to glVertexAttribPointer registered VBO as the currently bound vertex buffer object so afterwards we can safely unbind
	//
    // glBindVertexArray(0); // Unbind VAO (it's always a good thing to unbind any buffer/array to prevent strange bugs)

	// Projection matrix
	let proj = cgmath::perspective(cgmath::Deg(90.0 as f32), 8.0/6.0, 0.1, 100.0);
	let mut proj_loc = -1;
	unsafe { proj_loc = gl::GetUniformLocation(program.gl_id(), CString::new("matrix").unwrap().as_ptr()); }

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

	'running: loop {
		while let Some(event) = window.poll_event() {
			match event {
				Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
					break 'running
				},
				_ => {}
			}
		}

		window.clear();

		program.use_program();

		unsafe {
			// Set the projection matrix in the shader program.
			gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.as_ptr());

			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
	 	}

		window.display();
	}
}
