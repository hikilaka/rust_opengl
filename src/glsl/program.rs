use crate::glsl::{GLSLError, GLSLResult, Shader};
use crate::rsgl::{Bindable, Deletable};
use gl::types::*;
use nalgebra_glm as na;
use std::ops::Drop;

pub struct Program {
    pub handle: GLuint,
}

impl Bindable for Program {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Deletable for Program {
    fn delete(&mut self) {
        self.unbind();
        if self.handle != 0 {
            unsafe {
                gl::DeleteProgram(self.handle);
                self.handle = 0;
            }
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.delete();
    }
}

impl Program {
    pub fn make(shaders: &[Shader]) -> GLSLResult<Program> {
        use std::ptr::null_mut;

        let handle: GLuint;

        unsafe {
            handle = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(handle, shader.handle());
            }

            gl::LinkProgram(handle);

            let mut link_status: GLint = 0;
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut link_status);

            if link_status != (gl::TRUE as GLint) {
                let mut log_length: GLint = 0;
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut log_length);

                let mut buf = Vec::with_capacity(log_length as usize);
                buf.set_len((log_length - 1) as usize);
                gl::GetProgramInfoLog(
                    handle,
                    log_length,
                    null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let message =
                    std::str::from_utf8(&buf).map_err(|_| GLSLError::StringConversionError)?;
                return Err(GLSLError::ProgramLinkError {
                    message: message.to_owned(),
                });
            }

            for shader in shaders {
                gl::DetachShader(handle, shader.handle());
                gl::DeleteShader(shader.handle());
            }
        }

        Ok(Program { handle: handle })
    }

    unsafe fn find_uniform(&self, name: &str) -> GLSLResult<GLint> {
        use std::ffi::CString;

        let cname = CString::new(name.as_bytes()).map_err(|_| GLSLError::StringConversionError)?;
        let location = gl::GetUniformLocation(self.handle, cname.as_ptr());

        if location == -1 {
            return Err(GLSLError::UniformNotFoundError {
                name: name.to_owned(),
            });
        }

        Ok(location)
    }

    pub fn set_uniform1f(&self, name: &str, x: GLfloat) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::Uniform1f(location, x);
        }
        Ok(())
    }

    pub fn set_uniform2f(&self, name: &str, x: GLfloat, y: GLfloat) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::Uniform2f(location, x, y);
        }
        Ok(())
    }

    pub fn set_uniform3f(&self, name: &str, x: GLfloat, y: GLfloat, z: GLfloat) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::Uniform3f(location, x, y, z);
        }
        Ok(())
    }

    pub fn set_uniform4f(
        &self,
        name: &str,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::Uniform4f(location, x, y, z, w);
        }
        Ok(())
    }

    pub fn set_uniform_m4(&self, name: &str, matrix: &na::Mat4) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                na::value_ptr(matrix).as_ptr() as *const f32,
            );
        }
        Ok(())
    }

    pub fn set_uniform1i(&self, name: &str, x: GLint) -> GLSLResult<()> {
        unsafe {
            let location = self.find_uniform(name)?;
            gl::Uniform1i(location, x);
        }
        Ok(())
    }

    pub fn set_attribute(
        &self,
        name: &str,
        elements: usize,
        stride: usize,
        offset: usize,
    ) -> GLSLResult<()> {
        use std::ffi::CString;
        use std::mem::size_of;
        use std::os::raw::c_void;

        let cname = CString::new(name.as_bytes()).map_err(|_| GLSLError::StringConversionError)?;

        unsafe {
            let attrib_handle = gl::GetAttribLocation(self.handle, cname.as_ptr());

            if attrib_handle == -1 {
                return Err(GLSLError::AttributeNotFoundError {
                    name: name.to_owned(),
                });
            }
            gl::VertexAttribPointer(
                attrib_handle as GLuint,
                elements as GLint,
                gl::FLOAT,
                gl::FALSE,
                (stride * size_of::<GLfloat>()) as GLsizei,
                (offset * size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(attrib_handle as GLuint);
        }
        Ok(())
    }

    pub fn set_data_loc(&self, name: &str) -> GLSLResult<()> {
        use std::ffi::CString;

        let cname = CString::new(name.as_bytes()).map_err(|_| GLSLError::StringConversionError)?;

        unsafe { gl::BindFragDataLocation(self.handle, 0, cname.as_ptr()) }
        Ok(())
    }
}
