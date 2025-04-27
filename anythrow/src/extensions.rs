use std::{any::Any, panic::UnwindSafe};

use crate::{ResultLike, future::TryCatchFut};

pub trait UnwrapThrow {
    type Value;
    fn unwrap_throw(self) -> Self::Value;
}

impl<T> UnwrapThrow for Option<T> {
    type Value = T;

    /// Returns the contained `Some` value, consuming the self value.
    ///
    /// Throw a catchable error no value is contained.
    #[track_caller]
    fn unwrap_throw(self) -> Self::Value {
        match self {
            Some(v) => v,
            None => crate::throw_none(),
        }
    }
}

impl<T, E> UnwrapThrow for Result<T, E>
where
    E: Any + Send,
{
    type Value = T;

    /// Returns the contained `Ok` value, consuming the self value.
    ///
    /// If contain the `Err` variant, throw the contained error.
    #[track_caller]
    fn unwrap_throw(self) -> Self::Value {
        match self {
            Ok(v) => v,
            Err(err) => crate::throw(err),
        }
    }
}

pub trait TryCatchFutExt {
    fn try_catchable(self) -> TryCatchFut<Self>;
}

impl<F> TryCatchFutExt for F
where
    F: Future,
    F::Output: ResultLike,
    F: UnwindSafe,
{
    /// Wrap the future with a try/catch mechanism,
    /// correctly handling the error path in an async context.
    fn try_catchable(self) -> TryCatchFut<Self> {
        crate::try_catch_fut(self)
    }
}
