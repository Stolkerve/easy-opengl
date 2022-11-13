[![Crates.io](https://img.shields.io/crates/v/easy-opengl.svg)](https://crates.io/crates/easy-opengl)
### **easy-opengl** is a collection of utilities to make opengl app more quickly and easy without losing custumization and freedom. 

# Example
``` rust
use easy_opengl::buffers::*;
use easy_opengl::shader::*;
use easy_opengl::textures::*;

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
    uniform vec4 color = vec4(1.0);

    void main() {
        FragColor = texture(texture1, TexCoord) * color;
    }
"#;

pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}


pub fn main() {
    let mut shader = Shader::new();
    shader.load_from_memory(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None);

    let vertices = vec![
        Vertex {
            pos: [0.5, 0.5, 0.0],
            uv: [1.0, 1.0],
        }, // top right
        Vertex {
            pos: [0.5, -0.5, 0.0],
            uv: [1.0, 0.0],
        }, // bottom right
        Vertex {
            pos: [-0.5, -0.5, 0.0],
            uv: [0.0, 0.0],
        }, // bottom left
        Vertex {
            pos: [-0.5, 0.5, 0.0],
            uv: [0.0, 1.0],
        }, // top left
    ];
    let indices = vec![
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let vao = VertexArray::new();
    let _vbo = VertexBuffer::new(calc_bytes_size(&vertices) as isize, Some(&vertices));

    vao.bind();

    submit_vertex_attribs(&mut vec![
        VertexAttrib::new(VertexAttribType::Float3, false, "aPos".to_string()),
        VertexAttrib::new(VertexAttribType::Float2, false, "aTexCoord".to_string()),
    ]);

    let _ibo = IndexBuffer::new(calc_bytes_size(&indices) as isize, Some(&indices));

    let mut texture = Texture2D::new();
    texture.load_from_file("./a.png", TextureConfig::new());
}
```
