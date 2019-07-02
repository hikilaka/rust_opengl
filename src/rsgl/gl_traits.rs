use gl::types::GLenum;

pub trait Enum {
    fn value(&self) -> GLenum;
}

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}

pub trait Deletable {
    fn delete(&mut self);
}
