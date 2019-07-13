use crate::rsgl::{Bindable, Deletable};
use gl;
use gl::types::GLuint;
use std::ops::Drop;

#[derive(Clone)]
pub struct VertexArray {
    handle: GLuint,
}

impl Bindable for VertexArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }
    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Deletable for VertexArray {
    fn delete(&mut self) {
        self.unbind();
        if self.handle != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &mut self.handle);
            }
            self.handle = 0;
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        self.delete();
    }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut handle: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        VertexArray { handle }
    }
}
