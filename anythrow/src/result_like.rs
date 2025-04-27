use std::any::Any;

use crate::error::{OptionThrow, UnhandledError};

type BoxErr = Box<dyn Any + Send>;

pub trait ResultLike: Sized {
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr>;

    fn catch_or_resume(err: BoxErr) -> Self {
        match Self::try_from_err(err) {
            Ok(this) => this,
            Err(err) => std::panic::resume_unwind(err),
        }
    }
}

impl<T, E> ResultLike for Result<T, E>
where
    E: Any + Send,
{
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr> {
        match err.downcast::<UnhandledError<E>>() {
            Ok(err) => Ok(Err(err.into_inner())),
            Err(err) => Err(err),
        }
    }
}

impl<T> ResultLike for Option<T> {
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr> {
        match err.downcast::<OptionThrow>() {
            Ok(_) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
