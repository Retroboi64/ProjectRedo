#![allow(dead_code)]

use numix::types::{Mat4x4, Vec3};
use std::fs;

use gl::{FRAGMENT_SHADER, VERTEX_SHADER};

pub struct ShaderManager {
    shaders: Vec<Shader>,
    c_id: usize,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
            c_id: 0,
        }
    }

    pub fn add_shader(&mut self, name: &str, vert_path: &str, frag_path: &str) -> usize {
        let mut s = Shader::new(name);
        let (v, f) = s.compile_shaders_from_files(vert_path, frag_path);
        s.program = s.link_program(v, f);

        let id = self.shaders.len();
        self.shaders.push(s);
        self.c_id = id;
        id
    }

    pub fn set_current_id(&mut self, id: usize) {
        if id < self.shaders.len() {
            self.c_id = id;
        } else {
            println!(
                "Warning: shader id {} is out of range (have {})",
                id,
                self.shaders.len()
            );
        }
    }

    pub fn get_current_shader(&self) -> u32 {
        self.shaders
            .get(self.c_id)
            .expect("c_id points past end of shaders vec")
            .program
    }

    pub fn get_shader_by_id(&self, id: usize) -> &Shader {
        self.shaders
            .get(id)
            .unwrap_or_else(|| panic!("no shader with id {id}"))
    }
}

pub struct Shader {
    program: u32,
    name: String,
}

impl Shader {
    fn new(name: &str) -> Self {
        Self {
            program: 0,
            name: name.to_string(),
        }
    }

    fn load_shader_from_file(&self, path: &str) -> Vec<u8> {
        fs::read(path).unwrap_or_else(|e| panic!("failed to read shader '{path}': {e}"))
    }

    fn compile_shader(&self, src: &[u8], kind: gl::types::GLenum) -> u32 {
        let shader = unsafe { gl::CreateShader(kind) };
        let ptr = src.as_ptr().cast::<i8>();
        let len = src.len() as i32;
        unsafe {
            gl::ShaderSource(shader, 1, &ptr, &len);
            gl::CompileShader(shader);
        }

        let mut ok = 0i32;
        unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok) };
        if ok == 0 {
            let mut log_len = 0i32;
            unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len) };
            let mut buf = vec![0u8; log_len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    log_len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr().cast(),
                )
            };
            panic!("shader compile error: {}", String::from_utf8_lossy(&buf));
        }
        shader
    }

    fn compile_shaders_from_files(&self, vert_path: &str, frag_path: &str) -> (u32, u32) {
        let vs_src = self.load_shader_from_file(vert_path);
        let fs_src = self.load_shader_from_file(frag_path);
        let vs = self.compile_shader(&vs_src, VERTEX_SHADER);
        let fs = self.compile_shader(&fs_src, FRAGMENT_SHADER);
        (vs, fs)
    }

    fn link_program(&self, vert: u32, frag: u32) -> u32 {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);
        }

        let mut ok = 0i32;
        unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut ok) };
        if ok == 0 {
            let mut log_len = 0i32;
            unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len) };
            let mut buf = vec![0u8; log_len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    log_len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr().cast(),
                )
            };
            panic!("program link error: {}", String::from_utf8_lossy(&buf));
        }

        unsafe {
            gl::DeleteShader(vert);
            gl::DeleteShader(frag);
        }

        program
    }

    // Getters
    pub fn program(&self) -> u32 {
        self.program
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_uniform_vec3(&self, name: &str, v: Vec3<f32>) {
        let loc = unsafe { gl::GetUniformLocation(self.program, name.as_ptr().cast()) };
        if loc >= 0 {
            unsafe { gl::Uniform3f(loc, v.x, v.y, v.z) };
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, m: &Mat4x4<f32>) {
        let loc = unsafe { gl::GetUniformLocation(self.program, name.as_ptr().cast()) };
        if loc >= 0 {
            let col = m.as_col_major();
            unsafe { gl::UniformMatrix4fv(loc, 1, gl::FALSE, col.as_ptr()) };
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.program != 0 {
            unsafe { gl::DeleteProgram(self.program) };
        }
    }
}
