use std::{
    any::Any,
    panic::{UnwindSafe, catch_unwind},
};

mod error;
mod extensions;
mod future;
mod result_like;

use error::OptionThrow;
pub use error::UnhandledError;
pub use extensions::*;
pub use future::{TryCatchFut, try_catch_fut};
pub use result_like::ResultLike;

pub fn try_catch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
    F: UnwindSafe,
    R: ResultLike,
{
    match catch_unwind(f) {
        Ok(v) => v,
        Err(err) => R::catch_or_resume(err),
    }
}

/// Throw the error
#[inline(always)]
#[track_caller]
pub fn throw<T: Any + Send>(err: T) -> ! {
    std::panic::panic_any(error::UnhandledError::new(err))
}

/// Thow a special catchable error that signal a `None` value.
#[inline(always)]
#[track_caller]
pub fn throw_none() -> ! {
    std::panic::panic_any(OptionThrow::new())
}
