use thiserror::Error;

#[derive(Error, Debug)]
pub enum OICDError {
    #[error(transparent)]
    VerifyError(#[from] anyhow::Error),
    #[error("speciffic parameter required: {name}")]
    ParameterRequired { name: &'static str },
    #[error("email address is not registerd")]
    EmailNotRegisterd,
}

pub type OICDResult<T> = Result<T, OICDError>;
