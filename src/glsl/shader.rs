use crate::glsl::{GLSLError, GLSLResult};
use crate::rsgl::Enum;
use gl;
use gl::types::*;
use std::ops::Drop;

pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
}

impl Enum for ShaderType {
    fn value(&self) -> GLenum {
        match *self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
        }
    }
}

pub struct Shader {
    kind: ShaderType,
    handle: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.delete();
    }
}

impl Shader {
    pub fn make(kind: ShaderType, src: &str) -> GLSLResult<Shader> {
        use std::ffi::CString;
        use std::ptr::{null, null_mut};
        use std::str::from_utf8;

        let handle: GLuint;

        unsafe {
            let source =
                CString::new(src.as_bytes()).map_err(|_| GLSLError::StringConversionError)?;

            handle = gl::CreateShader(kind.value());

            gl::ShaderSource(handle, 1, &source.as_ptr(), null());
            gl::CompileShader(handle);

            let mut compile_status: GLint = 0;
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut compile_status);

            if compile_status != (gl::TRUE as GLint) {
                let mut log_length: GLint = 0;
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut log_length);

                let mut buf = Vec::with_capacity(log_length as usize);
                buf.set_len((log_length - 1) as usize);
                gl::GetShaderInfoLog(
                    handle,
                    log_length,
                    null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let message = from_utf8(&buf).map_err(|_| GLSLError::StringConversionError)?;
                return Err(GLSLError::ShaderCompilationError {
                    message: message.to_owned(),
                });
            }
        }

        Ok(Shader {
            kind: kind,
            handle: handle,
        })
    }

    pub fn kind(&self) -> &ShaderType {
        &self.kind
    }

    pub fn handle(&self) -> GLuint {
        self.handle
    }

    pub fn delete(&mut self) {
        if self.handle != 0 {
            unsafe {
                gl::DeleteShader(self.handle);
                self.handle = 0;
            }
        }
    }
}
