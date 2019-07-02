pub mod buffer;
pub mod gl_traits;
pub mod texture;
pub mod vertex_array;

#[macro_export]
macro_rules! bind {
    ($bindable:expr, $code:block) => {
        $bindable.bind();
        $code
        $bindable.unbind();
    };
    ($bindable:expr, unsafe $code:block) => {
        $bindable.bind();
        unsafe { $code }
        $bindable.unbind();
    };
}

pub use self::buffer::{Buffer, BufferType, BufferUsage};
pub use self::gl_traits::{Bindable, Deletable, Enum};
pub use self::vertex_array::VertexArray;
pub use self::texture::Texture;
