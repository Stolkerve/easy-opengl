use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::ptr;
use std::str;

use gl::types::*;

/// A abstract representation of a shader
///  # Example
/// ``` Rust
///
/// let mut shader1 = Shader::new();
/// shader1.load_from_memory(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None);
///
/// let mut shader2 = Shader::new();
/// shader2.load_from_file("./shaders/vertext.glsl", "./shaders/fragment.glsl", None);
///
/// shader1.set_uniform_int("entity_id", 33);
/// ```
pub struct Shader {
    pub program: u32,
    pub uniforms_location: HashMap<String, i32>,
}

impl Shader {
    pub fn new() -> Self {
        Self {
            program: 0,
            uniforms_location: HashMap::new(),
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program) }
    }

    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }

    pub fn load_from_memory(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
        geo_shader: Option<&String>,
    ) -> bool {
        let mut fail = false;

        let vertex = self.compile_shader(vertex_shader, 0, &mut fail);
        let fragment = self.compile_shader(fragment_shader, 1, &mut fail);
        let mut geo = None;
        if let Some(geo_shader) = geo_shader {
            geo = Some(self.compile_shader(geo_shader, 1, &mut fail));
        }

        let program = self.create_shader_program(&vertex, &fragment, &geo, &mut fail);
        self.delete_shaders(&vertex, &fragment, &geo);

        self.program = program;

        fail
    }

    pub fn load_from_file(
        &mut self,
        vertex_shader: &str,
        fragment_shader: &str,
        geo_shader: Option<&String>,
    ) -> bool {
        static read_files: fn(filename: &str) -> String = |filename: &str| -> String {
            let mut file = File::open(filename)
                .expect(format!("Couldn't open the file {}", filename).as_str());
            let mut source = String::new();
            file.read_to_string(&mut source)
                .expect("Couldn't read the file");
            source
        };

        let mut fail = false;
        let vertex_source = read_files(vertex_shader);
        let vertex = self.compile_shader(vertex_source.as_str(), 0, &mut fail);

        let fragment_source = read_files(fragment_shader);
        let fragment = self.compile_shader(fragment_source.as_str(), 1, &mut fail);

        let mut geo = None;
        if let Some(geo_shader) = geo_shader {
            let geo_source = read_files(geo_shader);
            geo = Some(self.compile_shader(geo_source.as_str(), 1, &mut fail));
        }

        let program = self.create_shader_program(&vertex, &fragment, &geo, &mut fail);
        self.delete_shaders(&vertex, &fragment, &geo);

        self.program = program;

        fail
    }

    pub fn set_uniform_int(&mut self, name: &str, v: &i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_locacion(name), *v);
        }
    }

    pub fn set_uniform_uint(&mut self, name: &str, v: &u32) {
        unsafe {
            gl::Uniform1ui(self.get_uniform_locacion(name), *v);
        }
    }

    pub fn set_uniform_float(&mut self, name: &str, v: &f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_locacion(name), *v);
        }
    }

    pub fn set_uniform_vec2(&mut self, name: &str, x: &f32, y: &f32) {
        unsafe {
            gl::Uniform2f(self.get_uniform_locacion(name), *x, *y);
        }
    }

    pub fn set_uniform_vec3(&mut self, name: &str, x: &f32, y: &f32, z: &f32) {
        unsafe {
            gl::Uniform3f(self.get_uniform_locacion(name), *x, *y, *z);
        }
    }

    pub fn set_uniform_vec4(&mut self, name: &str, x: &f32, y: &f32, z: &f32, w: &f32) {
        unsafe {
            gl::Uniform4f(self.get_uniform_locacion(name), *x, *y, *z, *w);
        }
    }

    pub fn set_uniform_mat3(&mut self, name: &str, m: *const f32) {
        unsafe {
            gl::UniformMatrix3fv(self.get_uniform_locacion(name), 1, gl::FALSE, m);
        }
    }

    pub fn set_uniform_mat4(&mut self, name: &str, m: *const f32) {
        unsafe {
            gl::UniformMatrix4fv(self.get_uniform_locacion(name), 1, gl::FALSE, m);
        }
    }

    fn get_uniform_locacion(&mut self, name: &str) -> i32 {
        if self.uniforms_location.contains_key(name) {
            return self.uniforms_location[name];
        }
        unsafe {
            let location = gl::GetUniformLocation(self.program, name.as_ptr() as *const i8);
            self.uniforms_location.insert(name.to_string(), location);
        }
        self.uniforms_location[name]
    }

    fn create_shader_program(
        &self,
        vertex: &u32,
        fragment: &u32,
        geo: &Option<u32>,
        fail: &mut bool,
    ) -> u32 {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, *vertex);
            gl::AttachShader(program, *fragment);
            if let Some(geo) = geo {
                gl::AttachShader(program, *geo);
            }
            gl::LinkProgram(program);

            let mut success = 0;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);

            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                *fail |= true;
                gl::GetProgramInfoLog(
                    program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            program
        }
    }

    fn compile_shader(&self, shader: &str, _type: u8, fail: &mut bool) -> u32 {
        unsafe {
            let id = gl::CreateShader(self.get_shader_type(&_type));

            let c_str_shader = CString::new(shader.as_bytes()).unwrap();
            gl::ShaderSource(id, 1, &c_str_shader.as_ptr(), ptr::null());
            gl::CompileShader(id);

            let mut success = 0;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);

            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                *fail |= true;

                gl::GetShaderInfoLog(
                    id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                println!(
                    "Compiling {} shader fail. Error: {}",
                    self.get_shader_name(&_type),
                    str::from_utf8(&info_log).unwrap()
                );
            }

            id
        }
    }

    fn delete_shaders(&self, vertex: &u32, fragment: &u32, geo: &Option<u32>) {
        unsafe {
            gl::DeleteShader(*vertex);
            gl::DeleteShader(*fragment);
            if let Some(geo) = geo {
                gl::DeleteShader(*geo);
            }
        }
    }

    fn get_shader_type(&self, _type: &u8) -> u32 {
        if *_type == 0 {
            gl::VERTEX_SHADER
        } else if *_type == 1 {
            gl::FRAGMENT_SHADER
        } else {
            gl::GEOMETRY_SHADER
        }
    }

    fn get_shader_name(&self, _type: &u8) -> &str {
        if *_type == 0 {
            "vertex"
        } else if *_type == 1 {
            "fragment"
        } else {
            "geometry"
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}
