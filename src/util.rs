use crate::glsl::{Program, Shader, ShaderType};
use gl;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

pub fn handle_event(event: Event) -> bool {
    match event {
        Event::Quit { .. } => return true,
        Event::KeyDown {
            keycode: Some(key), ..
        } => match key {
            Keycode::Escape | Keycode::Q => return true,
            Keycode::Num1 => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            },
            Keycode::Num2 => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            },
            _ => {}
        },
        Event::Window {
            win_event: WindowEvent::Resized(w, h),
            ..
        } => unsafe {
            gl::Viewport(0, 0, w, h);
        },
        _ => {}
    }
    false
}

pub fn load_program() -> Program {
    let vert_src = include_str!("../release/vert_shader.glsl");
    let frag_src = include_str!("../release/frag_shader.glsl");

    let vert_shader = match Shader::make(ShaderType::Vertex, vert_src) {
        Ok(shader) => shader,
        Err(e) => panic!("Error creating vertex shader: {}", e),
    };

    let frag_shader = match Shader::make(ShaderType::Fragment, frag_src) {
        Ok(shader) => shader,
        Err(e) => panic!("Error creating fragment shader: {}", e),
    };

    let program = match Program::make(&[vert_shader, frag_shader]) {
        Ok(program) => program,
        Err(e) => panic!("Error creating program: {}", e),
    };

    program
}
