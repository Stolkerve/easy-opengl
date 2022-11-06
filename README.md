[![Crates.io](https://img.shields.io/crates/v/easy-opengl.svg)](https://crates.io/crates/easy-opengl)
### **easy-opengl** is a collection of utilities to make opengl app more quickly and easy without losing custumization and freedom. 

# Example
``` rust
extern crate gl;
extern crate sdl2;

use easy_opengl::buffers::*;
use easy_opengl::shader::*;
use easy_opengl::textures::*;

use sdl2::event::Event;
use sdl2::event::WindowEvent;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;

    out vec2 TexCoord;

    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
       TexCoord = aTexCoord;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec2 TexCoord;

    uniform sampler2D texture1;

    void main() {
        FragColor = texture(texture1, TexCoord);
    }
"#;

pub struct Window {
    pub sdl_context: sdl2::Sdl,
    pub sdl_window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
    pub event_pump: sdl2::EventPump,
    pub name: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Titulo", width.clone(), height.clone())
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const std::ffi::c_void);
        let mut event_pump = sdl_context.event_pump().unwrap();

        Self {
            sdl_context,
            sdl_window: window,
            gl_context,
            event_pump,
            width,
            height,
            name,
        }
    }
}

pub fn main() {
    let mut window = Window::new(SCR_WIDTH, SCR_HEIGHT, "Test".to_string());
    let mut shader = Shader::new();
    shader.load_from_memory(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None);

    let vertices = vec![
        0.5, 0.5, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 0.0, 1.0, // top left
    ];
    let indices = vec![
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let vao = VertexArray::new();

    // Is important keep alive the variable, because when is out of scope it will destroy the buffer
    let _vbo = VertexBuffer::new(calc_bytes_size(&vertices) as isize, Some(&vertices));

    vao.bind();

    submit_vertex_attribs(&mut vec![
        VertexAttrib::new(VertexAttribType::Float3, false, "aPos".to_string()),
        VertexAttrib::new(VertexAttribType::Float2, false, "aTexCoord".to_string()),
    ]);

    // Is important keep alive the variable, because when is out of scope it will destroy the buffer
    let _ibo = IndexBuffer::new(calc_bytes_size(&indices) as isize, Some(&indices));

    let mut texture = Texture2D::new();
    texture.load_from_file("./a.png", TextureConfig::new());

    'main: loop {
        window.sdl_window.gl_swap_window();
        for event in window.event_pump.poll_iter() {
            unsafe {
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                texture.bind();
                shader.bind();
                vao.bind();
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
            match event {
                Event::Quit { .. } => {
                    break 'main;
                }
                Event::Window {
                    timestamp,
                    window_id,
                    win_event,
                } => match win_event {
                    WindowEvent::Resized(w, h) => unsafe {
                        gl::Viewport(0, 0, w, h);
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        // ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

```
