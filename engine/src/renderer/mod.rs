mod camera;
mod mesh;
mod scene;
mod shader;

use mesh::Mesh;
use numix::types::Mat4x4;
use numix::types::Vec3;
use scene::{NodeKind, Scene};
use shader::ShaderManager;

pub(crate) struct Renderer {
    m_shader: ShaderManager,
    scene: Scene,
    angle: f32,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {
            m_shader: ShaderManager::new(),
            scene: Scene::new(),
            angle: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.m_shader
            .add_shader("main", "./res/main.vert", "./res/main.frag");

        let root = self.scene.root_id();
        let pivot = self.scene.add_node("pivot", NodeKind::Empty, root);

        let cube_a =
            self.scene
                .add_node("cube_a", NodeKind::MeshNode(Mesh::cube("cube_a", 0)), pivot);
        self.scene.node_mut(cube_a).transform.position = Vec3::new(1.5, 0.0, 0.0);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn run(&mut self) {
        self.angle = (self.angle + 0.5) % 360.0;

        self.scene.node_mut(1).transform.rotation.y = self.angle.to_radians();

        self.scene.update();

        let aspect: f32 = 800.0 / 600.0;
        let cam_pos: Vec3<f32> = Vec3::new(0.0, 2.0, 6.0);
        let proj = perspective(45_f32.to_radians(), aspect, 0.1, 100.0);
        let view = look_at(cam_pos, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let view_proj = proj * view;

        let shader = self.m_shader.get_shader_by_id(0);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader.program());

            shader.set_uniform_vec3("uCamPos\0", cam_pos);
            shader.set_uniform_vec3("uLightDir\0", Vec3::new(0.6, 1.0, 0.8));
            shader.set_uniform_vec3("uLightColor\0", Vec3::new(1.0, 1.0, 0.95));
            shader.set_uniform_vec3("uBaseColor\0", Vec3::new(1.0, 0.55, 0.3));
        }

        self.scene.draw(shader.program(), view_proj);
    }
}

fn perspective(fovy_rad: f32, aspect: f32, near: f32, far: f32) -> Mat4x4<f32> {
    Mat4x4::perspective(fovy_rad, aspect, near, far)
}

fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4x4<f32> {
    let f = (target - eye).normalize();
    let r = f.cross(up).normalize();
    let u = r.cross(f);

    Mat4x4::from([
        [r.x, r.y, r.z, -r.dot(eye)],
        [u.x, u.y, u.z, -u.dot(eye)],
        [-f.x, -f.y, -f.z, f.dot(eye)],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
