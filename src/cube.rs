use nalgebra::{Point2, Point3, Translation3};

pub fn get_locs() -> Vec<Translation3<f32>> {
    let mut translations = Vec::new();
    translations.push(Translation3::new(0.0, 0.0, 0.0));
    translations.push(Translation3::new(2.0, 5.0, -15.0));
    translations.push(Translation3::new(-1.5, -2.2, -2.5));
    translations.push(Translation3::new(-3.8, -2.0, -12.3));
    translations.push(Translation3::new(2.4, -0.4, -3.5));
    translations.push(Translation3::new(-1.7, 3.0, -7.5));
    translations.push(Translation3::new(1.3, -2.0, -2.5));
    translations.push(Translation3::new(1.5, 2.0, -2.5));
    translations.push(Translation3::new(1.5, 0.2, -1.5));
    translations.push(Translation3::new(-1.3, 1.0, -1.5));

    translations.push(Translation3::new(-2.0, 4.0, 5.0));
    translations.push(Translation3::new(6.0, 12.2, 6.5));
    translations.push(Translation3::new(3.8, 2.0, 12.3));
    translations.push(Translation3::new(-2.4, 4.4, -3.5));
    translations.push(Translation3::new(3.7, 5.0, 2.5));
    translations.push(Translation3::new(6.3, -8.0, 9.5));
    translations.push(Translation3::new(8.5, 7.0, 3.5));
    translations.push(Translation3::new(9.5, 4.2, 6.5));
    translations.push(Translation3::new(11.3, 2.0, 8.5));

    translations.push(Translation3::new(2.0, 4.0, 5.0));
    translations.push(Translation3::new(-6.0, 12.2, 6.5));
    translations.push(Translation3::new(-3.8, 2.0, 12.3));
    translations.push(Translation3::new(2.4, 4.4, -3.5));
    translations.push(Translation3::new(-3.7, 5.0, 2.5));
    translations.push(Translation3::new(-6.3, -8.0, 9.5));
    translations.push(Translation3::new(-8.5, 7.0, 3.5));
    translations.push(Translation3::new(-9.5, 4.2, 6.5));
    translations.push(Translation3::new(-11.3, 2.0, 8.5));

    translations.push(Translation3::new(2.0, -4.0, 5.0));
    translations.push(Translation3::new(-6.0, -12.2, 6.5));
    translations.push(Translation3::new(-3.8, -2.0, 12.3));
    translations.push(Translation3::new(2.4, -4.4, -3.5));
    translations.push(Translation3::new(-3.7, -5.0, 2.5));
    translations.push(Translation3::new(-6.3, 8.0, 9.5));
    translations.push(Translation3::new(-8.5, -7.0, 3.5));
    translations.push(Translation3::new(-9.5, -4.2, 6.5));
    translations.push(Translation3::new(-11.3, -2.0, 8.5));

    translations.push(Translation3::new(2.0, 4.0, -5.0));
    translations.push(Translation3::new(-6.0, 12.2, -6.5));
    translations.push(Translation3::new(-3.8, 2.0, -12.3));
    translations.push(Translation3::new(2.4, 4.4, 3.5));
    translations.push(Translation3::new(-3.7, 5.0, -2.5));
    translations.push(Translation3::new(-6.3, -8.0, -9.5));
    translations.push(Translation3::new(-8.5, 7.0, -3.5));
    translations.push(Translation3::new(-9.5, 4.2, -6.5));
    translations.push(Translation3::new(-11.3, 2.0, -8.5));
    translations
}

pub fn get_verts() -> Vec<Point3<f32>> {
    let mut verts = Vec::new();
    verts.push(Point3::new(-0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, 0.5, -0.5));
    verts.push(Point3::new(0.5, 0.5, -0.5));
    verts.push(Point3::new(-0.5, 0.5, -0.5));
    verts.push(Point3::new(-0.5, -0.5, -0.5));

    verts.push(Point3::new(-0.5, -0.5, 0.5));
    verts.push(Point3::new(0.5, -0.5, 0.5));
    verts.push(Point3::new(0.5, 0.5, 0.5));
    verts.push(Point3::new(0.5, 0.5, 0.5));
    verts.push(Point3::new(-0.5, 0.5, 0.5));
    verts.push(Point3::new(-0.5, -0.5, 0.5));

    verts.push(Point3::new(-0.5, 0.5, 0.5));
    verts.push(Point3::new(-0.5, 0.5, -0.5));
    verts.push(Point3::new(-0.5, -0.5, -0.5));
    verts.push(Point3::new(-0.5, -0.5, -0.5));
    verts.push(Point3::new(-0.5, -0.5, 0.5));
    verts.push(Point3::new(-0.5, 0.5, 0.5));

    verts.push(Point3::new(0.5, 0.5, 0.5));
    verts.push(Point3::new(0.5, 0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, 0.5));
    verts.push(Point3::new(0.5, 0.5, 0.5));

    verts.push(Point3::new(-0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, -0.5));
    verts.push(Point3::new(0.5, -0.5, 0.5));
    verts.push(Point3::new(0.5, -0.5, 0.5));
    verts.push(Point3::new(-0.5, -0.5, 0.5));
    verts.push(Point3::new(-0.5, -0.5, -0.5));

    verts.push(Point3::new(-0.5, 0.5, -0.5));
    verts.push(Point3::new(0.5, 0.5, -0.5));
    verts.push(Point3::new(0.5, 0.5, 0.5));
    verts.push(Point3::new(0.5, 0.5, 0.5));
    verts.push(Point3::new(-0.5, 0.5, 0.5));
    verts.push(Point3::new(-0.5, 0.5, -0.5));
    verts
}

pub fn get_tex() -> Vec<Point2<f32>> {
    let mut tex_coords = Vec::new();
    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 0.0));

    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 0.0));

    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));

    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));

    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(0.0, 1.0));

    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords.push(Point2::new(1.0, 1.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(1.0, 0.0));
    tex_coords.push(Point2::new(0.0, 0.0));
    tex_coords.push(Point2::new(0.0, 1.0));
    tex_coords
}
