use crate::error::OptionThrowNone;
use std::{any::Any, task::Poll};

#[cfg(any(debug_assertions, feature = "debug"))]
use std::any::TypeId;

type BoxErr = Box<dyn Any + Send>;

pub trait ResultLike: Sized {
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr>;

    fn catch_or_resume(err: BoxErr) -> Self {
        match Self::try_from_err(err) {
            Ok(this) => this,
            Err(err) => std::panic::resume_unwind(err),
        }
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    fn catch_ids() -> impl IntoIterator<Item = TypeId>;
}

impl<T, E> ResultLike for Result<T, E>
where
    E: Any + Send,
{
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr> {
        match err.downcast::<E>() {
            Ok(err) => Ok(Err(*err)),
            Err(err) => Err(err),
        }
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    fn catch_ids() -> impl IntoIterator<Item = TypeId> {
        Some(TypeId::of::<E>())
    }
}

impl<T> ResultLike for Option<T> {
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr> {
        match err.downcast::<OptionThrowNone>() {
            Ok(_) => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    fn catch_ids() -> impl IntoIterator<Item = TypeId> {
        Some(TypeId::of::<OptionThrowNone>())
    }
}

impl<T> ResultLike for Poll<T>
where
    T: ResultLike,
{
    fn try_from_err(err: BoxErr) -> Result<Self, BoxErr> {
        T::try_from_err(err).map(Poll::Ready)
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    fn catch_ids() -> impl IntoIterator<Item = TypeId> {
        T::catch_ids()
    }
}
