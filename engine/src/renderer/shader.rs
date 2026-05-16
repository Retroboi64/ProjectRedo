use std::{fs, path::Path};

use gl::{FRAGMENT_SHADER, VERTEX_SHADER};

pub struct ShaderManager {
    shaders: Vec<Shader>,
    id: usize,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
            id: 0,
        }
    }

    pub fn add_shader(&mut self, name: &str, vert_path: &str, frag_path: &str) {
        let mut s = Shader::new(name);

        let (v, f) = s.compile_shaders_from_files(vert_path, frag_path);

        s.name = name.to_string();
        s.program = s.link_program(v, f);

        self.id += 1;
        self.shaders.push(s);
    }

    pub fn get_shader_by_id(&self, id: usize) -> u32 {
        let shader = self.shaders.get(id).unwrap();
        shader.program
    }
}

struct Shader {
    program: u32,
    name: String,
}

impl Shader {
    pub fn new(name: &str) -> Self {
        Self {
            program: 0u32,
            name: name.to_string(),
        }
    }

    pub fn load_shader_from_file(&self, path: &str) -> Vec<u8> {
        fs::read(path).unwrap()
    }

    fn compile_shader(&self, src: &[u8], kind: gl::types::GLenum) -> u32 {
        let shader = unsafe { gl::CreateShader(kind) };
        let ptr = src.as_ptr().cast::<i8>();
        let len = src.len() as i32;
        unsafe { gl::ShaderSource(shader, 1, &ptr, &len) };
        unsafe { gl::CompileShader(shader) };

        let mut ok = 0i32;
        unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok) };
        if ok == 0 {
            let mut len = 0i32;
            unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buf = vec![0u8; len as usize];
            unsafe {
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buf.as_mut_ptr().cast())
            };
            panic!("shader compile error: {}", String::from_utf8_lossy(&buf));
        }
        shader
    }

    fn compile_shaders_from_files(&self, vert_path: &str, frag_path: &str) -> (u32, u32) {
        let vs = self.load_shader_from_file(vert_path);
        let fs = self.load_shader_from_file(frag_path);

        let cvs = self.compile_shader(&vs, VERTEX_SHADER);
        let fvs = self.compile_shader(&fs, FRAGMENT_SHADER);

        (cvs, fvs)
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
            let mut len = 0i32;
            unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };
            let mut buf = vec![0u8; len as usize];
            unsafe {
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr().cast())
            };
            panic!("program link error: {}", String::from_utf8_lossy(&buf));
        }

        unsafe {
            gl::DeleteShader(vert);
            gl::DeleteShader(frag);
        }

        program
    }

    pub fn init(mut self, name: &str, vert: &[u8], frag: &[u8]) {
        let vert = self.compile_shader(vert, VERTEX_SHADER);
        let frag = self.compile_shader(frag, FRAGMENT_SHADER);

        self.program = self.link_program(vert, frag);
        self.name = name.to_string();
    }
}
