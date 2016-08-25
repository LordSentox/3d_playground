extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

pub mod graphics;
use graphics::*;

// Vertex data
static VERTEX_DATA: [GLfloat; 18] = [
	// Positions         // Colors
	 0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // Bottom Right
	-0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // Bottom Left
	 0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // Top
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

	unsafe { gl::Viewport(0, 0, 800, 600); }

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

		unsafe {
			program.use_program();

			gl::BindVertexArray(vao);
			gl::DrawArrays(gl::TRIANGLES, 0, 3);
	 	}

		window.display();
	}
}

/*
// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate glutin;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  1.0,
     1.0, -1.0,
    -1.0, -1.0
];

// Shader sources
static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
} }

fn main() {
    let window = glutin::Window::new().unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,
                                             CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }

    for event in window.wait_events() {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().unwrap();

        if let glutin::Event::Closed = event {
            break;
        }
    }

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

//! Working example of *obj-rs*
//! ========
//!
//! Execute it with the command below
//!
//!     cargo run --example glium --features glium-support

extern crate glium;
extern crate obj;

fn main() {
    use std::fs::File;
    use std::io::BufReader;
    use std::default::Default;
    use obj::*;
    use glium::{DisplayBuild, Program};

    // building the display, ie. the main object
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(500, 400)
        .with_title(format!("obj-rs"))
        .with_depth_buffer(32)
        .build_glium()
        .unwrap();

    let input = BufReader::new(File::open("data/test.obj").unwrap());
    let obj: Obj = load_obj(input).unwrap();

    let vb = obj.vertex_buffer(&display).unwrap();
    let ib = obj.index_buffer(&display).unwrap();

    let program = Program::from_source(&display, r#"
        #version 410
        uniform mat4 matrix;
        in vec3 position;
        in vec3 normal;
        smooth out vec3 _normal;
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
            _normal = normalize(normal);
        }
    "#, r#"
        #version 410
        uniform vec3 light;
        smooth in vec3 _normal;
        out vec4 result;
        void main() {
            result = vec4(clamp(dot(_normal, -light), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);
        }
    "#, None).unwrap();

    // drawing a frame
    let uniforms = uniform! {
        matrix: [
            [ 2.356724, 0.000000, -0.217148, -0.216930],
            [ 0.000000, 2.414214,  0.000000,  0.000000],
            [-0.523716, 0.000000, -0.977164, -0.976187],
            [ 0.000000, 0.000000,  9.128673,  9.219544f32]
        ],
        light: (-1.0, -1.0, -1.0f32)
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // the main loop
    // each cycle will draw once
    'main: loop {
        use glium::Surface;
        use std::thread::sleep;
        use std::time::Duration;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vb, &ib, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        // sleeping for some time in order not to use up too much CPU
        sleep(Duration::from_millis(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            use glium::glutin::Event::*;

            match event {
                Closed => break 'main,
                _ => ()
            }
        }
    }
}
*/
