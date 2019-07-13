pub mod cube;
pub mod glsl;
pub mod rsgl;
pub mod scene;
pub mod util;

use gl;
use nalgebra as na;
use rsgl::{Bindable, Texture};
use sdl2;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 650;

fn main() {
    use std::os::raw::c_void;

    let sdl = sdl2::init().expect("Error initializing SDL");
    let video = sdl.video().expect("Error creating video subsystem");

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_double_buffer(true);

    let window = video.window("Rust OpenGL", WIDTH, HEIGHT).opengl().build();

    let mut window = window.expect("Error creating window: {}");

    match window.set_minimum_size(WIDTH, HEIGHT) {
        Err(e) => panic!("Error window minimum size: {}", e.to_string()),
        _ => {}
    }

    let _ctx = window
        .gl_create_context()
        .expect("Error creating OpenGL context");

    let load_func = |s| video.gl_get_proc_address(s) as *const c_void;
    let _gl = gl::load_with(load_func);

    let mut event_pump = sdl.event_pump().expect("Error creating event pump");

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let program = util::load_program();
    let tex1 = Texture::new("release/container.jpg");
    let tex2 = Texture::new("release/awesomeface.png");

    let mut camera = scene::Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        std::f32::consts::FRAC_PI_4,
        0.1,
        100.0,
    );

    camera.look_at(na::Point3::origin());
    camera.set_position(na::Point3::new(0.0, 0.9, -3.0));

    let verts = cube::get_verts();
    let tex = cube::get_tex();
    let mesh = scene::Mesh::new(&verts, &tex);

    let mut models: Vec<scene::Model> = cube::get_locs()
        .iter()
        .map(|t| {
            let mut m = scene::Model::new(mesh.clone());
            m.translate(&t);
            m
        })
        .collect();

    let instant = std::time::Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            if util::handle_event(event) {
                break 'running;
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        tex1.activate_as(0);
        tex1.bind();
        tex2.activate_as(1);
        tex2.bind();

        let radius = 10.0;
        let cam_x = (instant.elapsed().as_millis() as f32 / 1400.0).sin() * radius;
        let cam_z = (instant.elapsed().as_millis() as f32 / 1000.0).cos() * radius;

        //camera.set_position(na::Point3::new(cam_x, 0.9, cam_z));

        for model in models.iter_mut() {
            model.rotate(
                std::f32::consts::FRAC_PI_3 * (instant.elapsed().as_millis() as f32 / 1000.0),
                scene::RotationAxis::Y,
            );
            model.render(&camera, &program);
        }

        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60))
    }
}
