use std::collections::VecDeque;

use numix::types::{Mat4x4, Vec3};

use crate::renderer::mesh::Mesh;

pub struct Transform {
    pub position: Vec3<f32>,
    pub rotation: Vec3<f32>,
    pub scale: Vec3<f32>,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vec3::new(0., 0., 0.),
            rotation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1., 1., 1.),
        }
    }

    pub fn to_mat4(&self) -> Mat4x4<f32> {
        let t = Mat4x4::translate(self.position.x, self.position.y, self.position.z);
        let rx = Mat4x4::rotate(self.rotation.x, 1.0, 0.0, 0.0);
        let ry = Mat4x4::rotate(self.rotation.y, 0.0, 1.0, 0.0);
        let rz = Mat4x4::rotate(self.rotation.z, 0.0, 0.0, 1.0);
        let s = Mat4x4::from([
            [self.scale.x, 0.0, 0.0, 0.0],
            [0.0, self.scale.y, 0.0, 0.0],
            [0.0, 0.0, self.scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        t * ry * rx * rz * s
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

pub enum NodeKind {
    Empty,
    MeshNode(Mesh),
}

pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub kind: NodeKind,
    pub visible: bool,

    parent: Option<usize>,
    children: Vec<usize>,

    world_transform: Mat4x4<f32>,
}

impl Node {
    fn new(name: impl Into<String>, kind: NodeKind) -> Self {
        Self {
            name: name.into(),
            transform: Transform::identity(),
            kind,
            visible: true,
            parent: None,
            children: Vec::new(),
            world_transform: Mat4x4::identity(),
        }
    }
}

pub struct Scene {
    nodes: Vec<Node>,
    root: usize,
}

impl Scene {
    pub fn new() -> Self {
        let root = Node::new("__root__", NodeKind::Empty);
        Self {
            nodes: vec![root],
            root: 0,
        }
    }

    pub fn add_node(&mut self, name: &str, kind: NodeKind, parent_id: usize) -> usize {
        assert!(parent_id < self.nodes.len(), "parent_id out of range");

        let id = self.nodes.len();
        let mut node = Node::new(name, kind);
        node.parent = Some(parent_id);
        self.nodes.push(node);
        self.nodes[parent_id].children.push(id);
        id
    }

    pub fn add_root_node(&mut self, name: &str, kind: NodeKind) -> usize {
        self.add_node(name, NodeKind::Empty, self.root);
        let root = self.root;
        self.add_node(name, kind, root)
    }

    pub fn remove_node(&mut self, id: usize) {
        if id == self.root {
            eprintln!("Cannot remove the scene root");
            return;
        }

        let mut to_remove = Vec::new();
        let mut queue = VecDeque::from([id]);
        while let Some(cur) = queue.pop_front() {
            to_remove.push(cur);
            for &child in &self.nodes[cur].children {
                queue.push_back(child);
            }
        }

        if let Some(parent_id) = self.nodes[id].parent {
            self.nodes[parent_id].children.retain(|&c| c != id);
        }

        for rid in to_remove {
            self.nodes[rid] = Node::new(format!("__removed_{rid}__"), NodeKind::Empty);
            self.nodes[rid].visible = false;
        }
    }

    pub fn node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn node_mut(&mut self, id: usize) -> &mut Node {
        &mut self.nodes[id]
    }

    pub fn world_transform(&self, id: usize) -> &Mat4x4<f32> {
        &self.nodes[id].world_transform
    }

    pub fn root_id(&self) -> usize {
        self.root
    }

    pub fn update(&mut self) {
        let mut queue = VecDeque::from([self.root]);
        while let Some(id) = queue.pop_front() {
            let local = self.nodes[id].transform.to_mat4();
            let world = if let Some(pid) = self.nodes[id].parent {
                self.nodes[pid].world_transform * local
            } else {
                local
            };
            self.nodes[id].world_transform = world;

            let children = self.nodes[id].children.clone();
            for child in children {
                queue.push_back(child);
            }
        }
    }

    pub fn draw(&self, program: u32, view_proj: Mat4x4<f32>) {
        let mvp_loc = unsafe { gl::GetUniformLocation(program, b"uMVP\0".as_ptr().cast()) };
        let model_loc = unsafe { gl::GetUniformLocation(program, b"uModel\0".as_ptr().cast()) };

        let mut queue = VecDeque::from([self.root]);
        while let Some(id) = queue.pop_front() {
            let node = &self.nodes[id];

            if node.visible {
                if let NodeKind::MeshNode(ref mesh) = node.kind {
                    let mvp = view_proj * node.world_transform;
                    let mvp_col = mvp.as_col_major();
                    let model_col = node.world_transform.as_col_major();

                    unsafe {
                        if mvp_loc >= 0 {
                            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp_col.as_ptr());
                        }
                        if model_loc >= 0 {
                            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_col.as_ptr());
                        }
                    }

                    mesh.draw();
                }

                for &child in &node.children {
                    queue.push_back(child);
                }
            }
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
