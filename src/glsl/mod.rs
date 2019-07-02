pub mod error;
pub mod program;
pub mod shader;

pub use self::error::{GLSLError, GLSLResult};
pub use self::program::Program;
pub use self::shader::{Shader, ShaderType};
