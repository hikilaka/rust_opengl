use crate::glsl::Program;
use crate::rsgl::Bindable;
use crate::scene::{Camera, Mesh};

use gl::types::GLsizei;
use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};

pub enum RotationAxis {
    X,
    Y,
    Z,
}

impl RotationAxis {
    pub fn value(&self) -> Vector3<f32> {
        match &self {
            RotationAxis::X => Vector3::x(),
            RotationAxis::Y => Vector3::y(),
            RotationAxis::Z => Vector3::z(),
        }
    }
}

pub struct Model {
    mesh: Box<Mesh>,
    translation: Translation3<f32>,
    rotation: UnitQuaternion<f32>,
}

impl Model {
    pub fn new(mesh: Mesh) -> Model {
        Model {
            mesh: Box::new(mesh),
            translation: Translation3::identity(),
            rotation: UnitQuaternion::identity(),
        }
    }

    pub fn rotate(&mut self, radians: f32, axis: RotationAxis) {
        let (mut roll, mut pitch, mut yaw) = self.rotation.euler_angles();
        match axis {
            RotationAxis::X => roll = radians,
            RotationAxis::Y => pitch = radians,
            RotationAxis::Z => yaw = radians,
        }
        self.rotation = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
        self.rotation.renormalize();
    }

    pub fn translate(&mut self, translation: &Translation3<f32>) {
        self.translation = Translation3::from(self.translation.vector + translation.vector);
    }

    pub fn render(&self, camera: &Camera, program: &Program) {
        program.bind();
        self.mesh.vert_arr.bind();

        let _ = program.set_data_loc("out_color");
        let _ = program.set_attribute("vert_pos", 3, 5, 0);
        let _ = program.set_attribute("tex_coord", 2, 5, 3);
        let _ = program.set_uniform1i("tex1", 0);
        let _ = program.set_uniform1i("tex2", 1);

        let _ = program.set_matrix("projection", camera.projection());
        let _ = program.set_matrix("view", &camera.view());

        let model = Isometry3::from_parts(self.translation.clone(), self.rotation.clone());
        let _ = program.set_matrix("model", &model.to_homogeneous());

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.vert_cnt as GLsizei);
        }

        self.mesh.vert_arr.unbind();
        program.unbind();
    }
}
