
#[derive(Debug, Clone)]
pub enum CustomError {
    EnvError(String),
    DotEnvError(String),
}

impl From<envy::Error> for CustomError {
    fn from(_value: envy::Error) -> Self {
        Self::EnvError("Error in parse .env at envy".into())
    }
}

impl From<dotenv::Error> for CustomError {
    fn from(_value: dotenv::Error) -> Self {
        Self::DotEnvError("Error in get at dotenv".into())
    }
}

impl From<serenity::Error> for CustomError {
    fn from(_value: serenity::Error) -> Self {
        Self::DotEnvError("Error in get at dotenv".into())
    }
}

pub type Result<T> = std::result::Result<T, CustomError>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;