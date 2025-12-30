use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsmRiscVError {
    #[error("")]
    NotImplementedInstruction,

    #[error("")]
    NotExistRegister,

    #[error("")]
    SyntaxError,

    #[error("")]
    ParseEmptyLine,

    #[error("")]
    ImmediateOverflow,

    #[error("")]
    ParseFunctError,

    #[error("")]
    UsedLabel,
}