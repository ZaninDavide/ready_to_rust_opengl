use glutin::{self, PossiblyCurrent};

use std::ffi::CStr;

/// Glwrapper safely provides the necessary functions to communicate with openGL
pub struct Glwrapper {
    pub gl: gl::Gl,
}

impl Glwrapper {
    pub fn new(gl_context: &glutin::Context<PossiblyCurrent>) -> Glwrapper {
        let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };
        println!("OpenGL version {}", version);

        Glwrapper { gl: gl }
    }

    pub fn draw_frame(&self, color: [f32; 4]) {
        unsafe {
            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
