mod shader;

use shader::ShaderManager;

#[rustfmt::skip]
const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0,   // bottom-left
     0.5, -0.5, 0.0,   // bottom-right
     0.0,  0.5, 0.0,   // top-center
];

pub(crate) struct Renderer {
    m_shader: ShaderManager,

    vao: u32,
    vbo: u32,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {
            m_shader: ShaderManager::new(),

            vao: 0u32,
            vbo: 0u32,
        }
    }

    pub fn init(&mut self) {
        self.m_shader
            .add_shader("main", "./res/main.vert", "./res/main.frag");

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * std::mem::size_of::<f32>()) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn run(&mut self) {
        let shader = self.m_shader.get_shader_by_id(0);
        unsafe {
            gl::ClearColor(0.15, 0.15, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
