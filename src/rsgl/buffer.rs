use crate::rsgl::{Bindable, Deletable, Enum};
use gl;
use gl::types::{GLenum, GLsizeiptr, GLuint};
use std::ops::Drop;

#[derive(Clone)]
pub enum BufferType {
    Array,
    ElementArray,
}

pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

impl Enum for BufferType {
    fn value(&self) -> GLenum {
        match self {
            BufferType::Array => gl::ARRAY_BUFFER,
            BufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl Enum for BufferUsage {
    fn value(&self) -> GLenum {
        match self {
            BufferUsage::Static => gl::STATIC_DRAW,
            BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
            BufferUsage::Stream => gl::STREAM_DRAW,
        }
    }
}

#[derive(Clone)]
pub struct Buffer {
    handle: GLuint,
    kind: BufferType,
}

impl Bindable for Buffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind.value(), self.handle);
        }
    }
    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.kind.value(), 0);
        }
    }
}

impl Deletable for Buffer {
    fn delete(&mut self) {
        self.unbind();
        if self.handle != 0 {
            unsafe {
                gl::DeleteBuffers(1, &mut self.handle);
            }
            self.handle = 0;
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.delete();
    }
}

impl Buffer {
    pub fn new(kind: BufferType) -> Buffer {
        let mut handle: GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        Buffer { handle, kind }
    }

    pub fn bind_data<T>(&self, usage: BufferUsage, data: &[T]) {
        use std::mem::size_of;
        use std::mem::transmute;

        self.bind();
        unsafe {
            gl::BufferData(
                self.kind.value(),
                (data.len() * size_of::<T>()) as GLsizeiptr,
                transmute(&data[0]),
                usage.value(),
            );
        }
    }

    pub fn handle(&self) -> GLuint {
        self.handle
    }
}
