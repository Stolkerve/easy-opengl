use std::ffi::c_void;

#[derive(Clone, Copy)]
pub enum VertexAttribType {
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Byte,
}

pub fn vertex_attrib_type_gl(vtype: &VertexAttribType) -> u32 {
    match vtype {
        VertexAttribType::Float => gl::FLOAT,
        VertexAttribType::Float2 => gl::FLOAT,
        VertexAttribType::Float3 => gl::FLOAT,
        VertexAttribType::Float4 => gl::FLOAT,
        VertexAttribType::Mat3 => gl::FLOAT,
        VertexAttribType::Mat4 => gl::FLOAT,
        VertexAttribType::Int => gl::INT,
        VertexAttribType::Int2 => gl::INT,
        VertexAttribType::Int3 => gl::INT,
        VertexAttribType::Int4 => gl::INT,
        VertexAttribType::Byte => gl::BYTE,
    }
}

pub fn vertex_attrib_type_size(vtype: &VertexAttribType) -> u32 {
    match vtype {
        VertexAttribType::Float => 4,
        VertexAttribType::Float2 => 4 * 2,
        VertexAttribType::Float3 => 4 * 3,
        VertexAttribType::Float4 => 4 * 4,
        VertexAttribType::Mat3 => 4 * 3 * 3,
        VertexAttribType::Mat4 => 4 * 4 * 4,
        VertexAttribType::Int => 4,
        VertexAttribType::Int2 => 4 * 2,
        VertexAttribType::Int3 => 4 * 3,
        VertexAttribType::Int4 => 4 * 4,
        VertexAttribType::Byte => 1,
    }
}

pub fn vertex_attrib_type_count(vtype: &VertexAttribType) -> u32 {
    match vtype {
        VertexAttribType::Float => 1,
        VertexAttribType::Float2 => 2,
        VertexAttribType::Float3 => 3,
        VertexAttribType::Float4 => 4,
        VertexAttribType::Mat3 => 3 * 3,
        VertexAttribType::Mat4 => 4 * 4,
        VertexAttribType::Int => 1,
        VertexAttribType::Int2 => 2,
        VertexAttribType::Int3 => 3,
        VertexAttribType::Int4 => 4,
        VertexAttribType::Byte => 1,
    }
}

/// A abstract representation of a vertex attribute
///
/// # Example
///
/// ``` Rust
/// let attrib = VertexAttrib::new(
///       VertexAttribType::Float3, // We want to send a vector3
///       false, // normalize
///       "pos".to_string(), // the name is only to know the which attribute is to the end user
/// )
/// ```
pub struct VertexAttrib {
    pub size: u32,
    pub offset: u32,
    pub vtype: VertexAttribType,
    pub normalize: bool,
    pub name: String,
}

impl VertexAttrib {
    pub fn new(vtype: VertexAttribType, normalize: bool, name: String) -> Self {
        Self {
            size: vertex_attrib_type_size(&vtype),
            offset: 0,
            vtype,
            normalize,
            name,
        }
    }
}

/// Attach the vector of vertex attributes to a binded vertex array
///
/// # Example
/// ``` Rust
///    let vertices = vec![
///        0.5, 0.5, 0.0, // top right
///        0.5, -0.5, 0.0, // bottom right
///        -0.5, -0.5, 0.0, // bottom left
///        -0.5, 0.5, 0.0, // top left
///    ];
///    let vao = VertexArray::new();
///    vao.bind();
///    submit_vertex_attribs(&mut vec![VertexAttrib::new(
///        VertexAttribType::Float3,
///        false,
///        "pos".to_string(),
///    )]);
/// ```
pub fn submit_vertex_attribs(vertex_attribs: &mut Vec<VertexAttrib>) {
    let mut stride = 0;
    let mut offset = 0;
    for attrib in vertex_attribs.iter_mut() {
        attrib.offset += offset;
        offset += attrib.size;
        stride += attrib.size;
    }

    let mut i = 0;
    for attrib in vertex_attribs {
        if vertex_attrib_type_gl(&attrib.vtype) == gl::FLOAT {
            unsafe {
                gl::VertexAttribPointer(
                    i,
                    vertex_attrib_type_count(&attrib.vtype) as i32,
                    vertex_attrib_type_gl(&attrib.vtype),
                    attrib.normalize as u8,
                    stride as i32,
                    attrib.offset as *const std::ffi::c_void,
                );
            }
        } else {
            unsafe {
                gl::VertexAttribIPointer(
                    i,
                    vertex_attrib_type_count(&attrib.vtype) as i32,
                    vertex_attrib_type_gl(&attrib.vtype),
                    stride as i32,
                    attrib.offset as *const std::ffi::c_void,
                );
            }
        }

        unsafe {
            gl::EnableVertexAttribArray(i);
        }

        i += 1;
    }
}

fn gen_vao() -> u32 {
    let mut vao: u32 = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) }
    vao
}

fn gen_buffer() -> u32 {
    let mut buffer: u32 = 0;
    unsafe { gl::GenBuffers(1, &mut buffer) }
    buffer
}

pub fn calc_bytes_size<T>(v: &Vec<T>) -> usize {
    v.len() * std::mem::size_of::<T>()
}

/// A abstract representation of a vertex array
/// # Example
/// ``` Rust
///    let vertices = vec![
///        0.5, 0.5, 0.0, // top right
///        0.5, -0.5, 0.0, // bottom right
///        -0.5, -0.5, 0.0, // bottom left
///        -0.5, 0.5, 0.0, // top left
///    ];
///    let indices = vec![
///        0, 1, 3, // first Triangle
///        1, 2, 3, // second Triangle
///    ];
///
///    let vao = VertexArray::new();
///    let _vbo = VertexBuffer::new(calc_bytes_size(&vertices) as isize, Some(&vertices));
///
///    vao.bind();
///
///    submit_vertex_attribs(&mut vec![VertexAttrib::new(
///        VertexAttribType::Float3,
///        false,
///        "pos".to_string(),
///    )]);
///
///    let _ibo = IndexBuffer::new(calc_bytes_size(&indices) as isize, Some(&indices));
/// ```
pub struct VertexArray {
    pub id: u32,
}

impl VertexArray {
    /// Return a vertext array

    pub fn new() -> Self {
        Self { id: gen_vao() }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

/// A abstract representation of a vertex buffer
///  # Example
/// ``` Rust
///    let vertices = vec![
///        0.5, 0.5, 0.0, // top right
///        0.5, -0.5, 0.0, // bottom right
///        -0.5, -0.5, 0.0, // bottom left
///        -0.5, 0.5, 0.0, // top left
///    ];
///
///    // static
///    let vbo1 = VertexBuffer::new(calc_bytes_size(&vertices) as isize, Some(&vertices));
///
///    // Dynamic
///    let vbo2 = VertexBuffer::new(calc_bytes_size(&vertices) as isize);
///
///    // send half of the vertices
///    vbo2.send_data(48 / 2, 0, vertices);
/// ```
pub struct VertexBuffer {
    pub id: u32,
}

impl VertexBuffer {
    /// Return a VertexBuffer with the allocated size provided, the buffer data is static only if
    /// the verticies isn't None, else, the buffer data is dynamic
    ///
    ///  # Arguments
    ///  * `size` - The size in bytes of the data to allocate
    ///  * `vertices` - A optional data to write

    pub fn new(size: isize, vertices: Option<&Vec<f32>>) -> Self {
        let _self = Self { id: gen_buffer() };
        _self.bind();

        if let Some(vertices) = vertices {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    size,
                    vertices.as_ptr() as *const std::ffi::c_void,
                    gl::STATIC_DRAW,
                );
            }
        } else {
            unsafe {
                gl::BufferData(gl::ARRAY_BUFFER, size, std::ptr::null(), gl::DYNAMIC_DRAW);
            }
        }

        _self
    }

    /// Write data that wasn't provided on the new function
    ///
    ///  # Arguments
    ///  * `size` - The size in bytes of the data to write
    ///  * `offset` - Point to a offset in the allocated space
    ///  * `vertices` - Data to write
    pub fn send_data(&self, size: isize, offset: isize, vertices: &Vec<f32>) {
        unsafe {
            self.bind();
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                offset,
                size,
                vertices.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

/// A abstract representation of a index buffer
///  # Example
/// ``` Rust
///    let indices = vec![
///        0, 1, 3, // first Triangle
///        1, 2, 3, // second Triangle
///    ];
///
///    // static
///    let ibo1 = VertexBuffer::new(calc_bytes_size(&indices) as isize, Some(&vertices));
///
///    // Dynamic
///    let ibo2 = VertexBuffer::new(calc_bytes_size(&indices) as isize);
///
///    // send half of the vertices
///    ibo2.send_data(24 / 2, 0, &indices);
/// ```
pub struct IndexBuffer {
    pub id: u32,
}

impl IndexBuffer {
    /// Return a IndexBuffer with the allocated size provided, the buffer data is static only if
    /// the indices isn't None, else, the buffer data is dynamic
    ///
    ///  # Arguments
    ///  * `size` - The size in bytes of the data to allocate
    ///  * `indices` - A optional data to write
    pub fn new(size: isize, indices: Option<&Vec<i32>>) -> Self {
        let _self = Self { id: gen_buffer() };
        _self.bind();

        if let Some(indices) = indices {
            unsafe {
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    size,
                    indices.as_ptr() as *const std::ffi::c_void,
                    gl::STATIC_DRAW,
                );
            }
        } else {
            unsafe {
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    size,
                    std::ptr::null(),
                    gl::DYNAMIC_DRAW,
                );
            }
        }
        _self
    }

    /// Write data that wasn't provided on the new function
    ///
    ///  # Arguments
    ///  * `size` - The size in bytes of the data to write
    ///  * `offset` - Point to a offset in the allocated space
    ///  * `indices` - Data to write
    pub fn send_data(&self, size: isize, offset: isize, indices: &Vec<i32>) {
        unsafe {
            self.bind();
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                offset,
                size,
                indices.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

/// A abstract representation of a dynamic uniform buffer
pub struct UniforBuffer {
    pub id: u32,
    pub slot: u32,
}

impl UniforBuffer {
    /// # Arguments
    /// * `size` - Size in bytes of the buffer
    /// * `binding` - The binding slot
    pub fn new(size: isize, binding: u32) -> Self {
        let _self = Self {
            id: gen_buffer(),
            slot: binding,
        };
        unsafe {
            gl::BufferData(gl::UNIFORM_BUFFER, size, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, _self.slot, _self.id);
        }

        _self
    }

    /// # Arguments
    /// * `data` - A void ptr to a array of data
    /// * `size` - Size in bytes of the buffer
    /// * `offset` - Offset pointing on the allocated data
    pub fn send_data(&self, data: *const c_void, size: isize, offset: isize) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, self.slot, self.id);
            gl::BufferSubData(gl::UNIFORM_BUFFER, offset, size, data);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for UniforBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
