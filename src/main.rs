pub mod glsl;
pub mod rsgl;
pub mod util;

use gl;
use nalgebra_glm as na;
use rsgl::{Bindable, Buffer, BufferType, BufferUsage, Texture, VertexArray};
use sdl2;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 650;

const VERTICIES: [gl::types::GLfloat; 180] = [
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
    -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5, 0.0,
    0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5,
    0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5,
    0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0,
    0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5,
    0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5,
    -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5,
    1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5,
    0.5, -0.5, 0.0, 1.0,
];

fn main() {
    use std::os::raw::c_void;

    let sdl = sdl2::init().expect("Error initializing SDL");
    let video = sdl.video().expect("Error creating video subsystem");

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 2);
    gl_attr.set_double_buffer(true);

    let window = video
        .window("Rust OpenGL", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build();

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
    let program = util::load_program();

    let tex1 = Texture::new("release/container.jpg");
    let tex2 = Texture::new("release/awesomeface.png");

    let vert_array = VertexArray::new();
    let vert_buffer = Buffer::new(BufferType::Array);

    bind!(vert_array, {
        vert_buffer.bind_data(BufferUsage::Static, &VERTICIES);

        program.bind();

        let _ = program.set_data_loc("out_color");
        let _ = program.set_attribute("vert_pos", 3, 5, 0);
        let _ = program.set_attribute("tex_coord", 2, 5, 3);
        let _ = program.set_uniform1i("tex1", 0);
        let _ = program.set_uniform1i("tex2", 1);
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let cube_locs = [
        na::vec3(0.0, 0.0, 0.0),
        na::vec3(2.0, 5.0, -15.0),
        na::vec3(-1.5, -2.2, -2.5),
        na::vec3(-3.8, -2.0, -12.3),
        na::vec3(2.4, -0.4, -3.5),
        na::vec3(-1.7, 3.0, -7.5),
        na::vec3(1.3, -2.0, -2.5),
        na::vec3(1.5, 2.0, -2.5),
        na::vec3(1.5, 0.2, -1.5),
        na::vec3(-1.3, 1.0, -1.5),
    ];

    let instant = std::time::Instant::now();

    let cam_pos = na::vec3(0.0, 0.0, 0.3);
    let cam_target = na::vec3(0.0, 0.0, 0.0);
    let cam_dir = na::normalize(&(cam_pos - cam_target));
    let up = na::vec3(0.0, 1.0, 0.0);
    let cam_right = na::normalize(&up.cross(&cam_dir));
    let cam_up = cam_dir.cross(&cam_right);

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

        let fov = na::radians(&na::vec1(45.0))[0];
        let proj = na::perspective(WIDTH as f32 / HEIGHT as f32, fov, 0.1, 100.0);

        // let view: na::Mat4 = na::translate(&na::identity(), &na::vec3(0.0, 0.0, -3.0));
        let radius = 10.0;
        let cam_x = (instant.elapsed().as_millis() as f32 / 1000.0).sin() * radius;
        let cam_z = (instant.elapsed().as_millis() as f32 / 1000.0).cos() * radius;
        let view = na::look_at(
            &na::vec3(cam_x, 0.0, cam_z),
            &na::vec3(0.0, 0.0, 0.0),
            &na::vec3(0.0, 1.0, 0.0),
        );

        let _ = program.set_uniform_m4("projection", &proj);
        let _ = program.set_uniform_m4("view", &view);

        for (i, cube) in cube_locs.iter().enumerate() {
            let rotation = na::radians(&na::vec1(20.0 * i as f32))[0];
            let mut model = na::translate(&na::identity(), &cube);
            model = na::rotate(&model, rotation, &na::vec3(1.0, 0.3, 0.5));
            let _ = program.set_uniform_m4("model", &model);

            bind!(vert_array, unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            });
        }

        window.gl_swap_window();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60))
    }
}
