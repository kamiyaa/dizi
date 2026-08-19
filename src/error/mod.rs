mod error_kind;
mod error_type;

pub use self::error_kind::DiziErrorKind;
pub use self::error_type::DiziError;

pub type AppResult<T = ()> = Result<T, DiziError>;
