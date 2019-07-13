use crate::rsgl::{Bindable, Buffer, BufferType, BufferUsage, VertexArray};

use nalgebra::{Point2, Point3};

#[derive(Clone)]
pub struct Mesh {
    pub vert_arr: VertexArray,
    pub vert_buf: Buffer,
    pub vert_cnt: usize,
}

type Vec3f = Vec<Point3<f32>>;
type Vec2f = Vec<Point2<f32>>;

impl Mesh {
    fn buffer_data(verticies: &Vec3f, tex_coords: &Vec2f) -> Vec<f32> {
        assert!(verticies.len() == tex_coords.len());

        // formatted: x,y,z,u,v
        let mut data: Vec<f32> = Vec::with_capacity(verticies.len() * 5);

        data.resize(verticies.len() * 5, 0.0);

        for (i, vert) in verticies.iter().enumerate() {
            let tex = tex_coords[i];
            let offset = i * 5;

            data[offset + 0] = vert.coords[0];
            data[offset + 1] = vert.coords[1];
            data[offset + 2] = vert.coords[2];
            data[offset + 3] = tex.coords[0];
            data[offset + 4] = tex.coords[1];
        }
        data
    }

    pub fn new(verticies: &Vec<Point3<f32>>, tex_coords: &Vec<Point2<f32>>) -> Mesh {
        let data = Self::buffer_data(&verticies, &tex_coords);

        let vert_arr = VertexArray::new();
        let vert_buf = Buffer::new(BufferType::Array);

        vert_arr.bind();
        vert_buf.bind_data(BufferUsage::Static, &data);
        vert_arr.unbind();

        Mesh {
            vert_arr: vert_arr,
            vert_buf: vert_buf,
            vert_cnt: verticies.len(),
        }
    }
}
