use std::ffi::c_void;
use std::ffi::CString;

#[derive(Copy, Clone)]
pub enum TextureParam {
    Linear = gl::LINEAR as isize,
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    Nearest = gl::NEAREST as isize,
    Repeat = gl::REPEAT as isize,
}

#[derive(Clone, Copy)]
pub enum TextureFormat {
    Rgba = gl::RGBA as isize,
    Rgb = gl::RGB as isize,
    Red = gl::RED as isize,
}

// pub enum PixelDataType {
//     I8 = gl::BYTE as isize,
//     U8 = gl::UNSIGNED_BYTE as isize,
//     I32 = gl::INT as isize,
//     U32 = gl::UNSIGNED_INT as isize,
//     F32 = gl::FLOAT as isize,
// }

pub struct TextureConfig {
    min_filter: TextureParam,
    mag_filter: TextureParam,
    wrap_s: TextureParam,
    wrap_t: TextureParam,

    format: TextureFormat,
    internal_format: TextureFormat,
    // pixel_data_type: PixelDataType,
    bitmap: bool,
}

impl TextureConfig {
    pub fn new() -> Self {
        Self {
            min_filter: TextureParam::Nearest,
            mag_filter: TextureParam::Linear,
            wrap_s: TextureParam::Repeat,
            wrap_t: TextureParam::Repeat,
            format: TextureFormat::Rgb,
            internal_format: TextureFormat::Rgba,
            // pixel_data_type: PixelDataType::U8,
            bitmap: true,
        }
    }
}

/// A abstract representation of a 2D texture
///  # Example
/// ``` Rust
/// let mut texture1 = Texture2D::new();
/// texture1.load_from_file("./src/a.png", TextureConfig::new());
/// texture1.send_data(30, 30, 1, 1, 0xFF000000 as ptr); // Set a red pixel on x: 30, y: 30
///
/// let data = vec![...];
/// let texture2 = Texture2D::new();
/// texture2.gen_texture(TextureConfig::new());
/// texture2.send_data(0, 0, 100, 200, data as ptr);
///
/// let texture3 = Texture2D::new();
/// texture3.load_from_memory(data as ptr, TextureConfig::new());
/// ```
pub struct Texture2D {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub config: Option<TextureConfig>,
}

impl Texture2D {
    pub fn new() -> Self {
        Self {
            id: 0,
            width: 0,
            height: 0,
            config: None,
        }
    }
    // Its function allow to generate and allocate a texture to send data later
    pub fn gen_texture(&mut self, config: TextureConfig) {
        if self.config.is_some() {
            println!("Texture already created");
            return;
        }

        self.config = Some(config);
        let config = self.config.as_mut().unwrap();
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                config.min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                config.mag_filter as i32,
            );
        }
    }

    // Send data on a already allocated texture with the config of the generated texture
    pub fn send_data(
        &self,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        data: *const c_void,
    ) {
        if self.config.is_none() {
            println!("A texture needs to be created first");
            return;
        }

        let config = self.config.as_ref().unwrap();

        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                xoffset as i32,
                yoffset as i32,
                width as i32,
                height as i32,
                config.format as u32,
                gl::UNSIGNED_BYTE,
                data,
            );
        }
    }

    // Generate and allocate a texture with the given data
    pub fn load_from_memory(
        &mut self,
        width: u32,
        height: u32,
        data: *const c_void,
        config: TextureConfig,
    ) {
        if self.config.is_some() {
            println!("Texture already created");
            return;
        }

        self.config = Some(config);
        let config = self.config.as_mut().unwrap();

        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                config.min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                config.mag_filter as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                config.internal_format as i32,
                width as i32,
                height as i32,
                0,
                config.format as u32,
                gl::UNSIGNED_BYTE,
                data,
            );
        }
    }

    // Generate and allocate a texture with given file path
    pub fn load_from_file(&mut self, filepath: &str, config: TextureConfig) {
        if self.config.is_some() {
            println!("Texture already created");
            return;
        }

        self.config = Some(config);
        let config = self.config.as_mut().unwrap();

        unsafe {
            let c_str_filename = CString::new(filepath.as_bytes()).unwrap();
            stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
            let mut width = 0;
            let mut height = 0;
            let mut channels = 0;
            let data = stb_image::stb_image::bindgen::stbi_load(
                c_str_filename.as_ptr(),
                &mut width,
                &mut height,
                &mut channels,
                0,
            );

            if data.is_null() {
                panic!("Fail to load texture {}", filepath);
            }

            if channels == 1 {
                config.format = TextureFormat::Red;
            } else if channels == 3 {
                config.format = TextureFormat::Rgb;
            } else {
                config.format = TextureFormat::Rgba;
            }

            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                config.min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                config.mag_filter as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                config.internal_format as i32,
                width,
                height,
                0,
                config.format as u32,
                gl::UNSIGNED_BYTE,
                data as *const c_void,
            );

            if config.bitmap {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
