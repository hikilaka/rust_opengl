use nalgebra::{Isometry3, Matrix4, Perspective3, Point, Point3, Vector3};

pub struct Camera {
    pub projection: Perspective3<f32>,
    pub view: Isometry3<f32>,
    pub position: Point3<f32>,
    pub target: Point3<f32>,
}

impl Camera {
    pub fn new(aspect: f32, fov: f32, znear: f32, zfar: f32) -> Camera {
        Camera {
            projection: Perspective3::new(aspect, fov, znear, zfar),
            view: Isometry3::identity(),
            position: Point3::origin(),
            target: Point3::origin(),
        }
    }

    pub fn set_projection(&mut self, aspect: f32, fov: f32, znear: f32, zfar: f32) {
        self.projection = Perspective3::new(aspect, fov, znear, zfar);
    }

    // TODO: better optimize this to modify the view directly instead
    // of just creating a new object
    pub fn set_position(&mut self, location: Point3<f32>) {
        self.position = location;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &Vector3::y());
    }

    pub fn offset_position(&mut self, offset: Point3<f32>) {
        let new_position = Point::from(self.position.to_homogeneous() + offset.to_homogeneous());
        self.set_position(new_position.xyz());
    }

    pub fn look_at(&mut self, location: Point3<f32>) {
        self.target = location;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &Vector3::y());
    }

    pub fn projection(&self) -> &Matrix4<f32> {
        &self.projection.as_matrix()
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view.to_homogeneous()
    }

    pub fn position(&self) -> &Point3<f32> {
        &self.position
    }

    pub fn target(&self) -> &Point3<f32> {
        &self.target
    }
}
