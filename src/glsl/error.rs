use failure::Fail;

#[derive(Debug, Fail)]
pub enum GLSLError {
    #[fail(display = "Failed to convert str to CString")]
    StringConversionError,

    #[fail(display = "Shader compile error: {}", message)]
    ShaderCompilationError { message: String },

    #[fail(display = "Program link error: {}", message)]
    ProgramLinkError { message: String },

    #[fail(display = "Uniform '{}' not found in program", name)]
    UniformNotFoundError { name: String },

    #[fail(display = "Attribute '{}' not found in program", name)]
    AttributeNotFoundError { name: String },
}

pub type GLSLResult<T> = Result<T, GLSLError>;
