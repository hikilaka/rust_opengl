use crate::rsgl::{Bindable, Deletable};
use gl;
use gl::types::GLuint;
use image;
use image::GenericImageView;
use std::ops::Drop;

pub struct Texture {
    handle: GLuint,
}

impl Bindable for Texture {
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }
    fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Deletable for Texture {
    fn delete(&mut self) {
        self.unbind();
        if self.handle != 0 {
            unsafe {
                gl::DeleteTextures(1, &mut self.handle);
            }
            self.handle = 0;
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.delete();
    }
}

impl Texture {
    pub fn new(path: &str) -> Texture {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e) => panic!("Error loading texture '{}': {}", path, e),
        };
        let data = img.raw_pixels();
        let format = match img {
            image::ImageLuma8(_) => gl::RED,
            image::ImageLumaA8(_) => gl::RG,
            image::ImageRgb8(_) => gl::RGB,
            image::ImageRgba8(_) => gl::RGBA,
            image::ImageBgr8(_) => gl::BGR,
            image::ImageBgra8(_) => gl::BGRA,
        };

        let mut handle: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const std::os::raw::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { handle }
    }

    pub fn activate_as(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
        }
    }
}
