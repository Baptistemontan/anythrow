use std::any::type_name_of_val;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::panic::Location;

pub struct UnhandledError<T: ?Sized> {
    location: &'static Location<'static>,
    err: T,
}

impl<T> UnhandledError<T> {
    pub const fn new(err: T) -> Self {
        UnhandledError {
            location: Location::caller(),
            err,
        }
    }

    pub fn into_inner(self) -> T {
        self.err
    }
}

impl<T: ?Sized> Debug for UnhandledError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_name = type_name_of_val(&self.err);
        f.debug_struct("UnhandledError")
            .field("location", self.location)
            .field("err", &err_name)
            .finish()
    }
}

impl<T: ?Sized> Display for UnhandledError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_name = type_name_of_val(&self.err);
        write!(
            f,
            "Unhandled error of type {} created at {}.",
            err_name, self.location
        )
    }
}

impl<T> Error for UnhandledError<T> {}

pub(crate) struct OptionThrow {
    _marker: (),
    location: &'static Location<'static>,
}

impl OptionThrow {
    pub const fn new() -> Self {
        OptionThrow {
            _marker: (),
            location: Location::caller(),
        }
    }
}

const OPTION_UNWRAP_MSG: &str = "called `Option::unwrap()` on a `None` value";

impl Debug for OptionThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionThrow")
            .field("msg", &OPTION_UNWRAP_MSG)
            .field("location", self.location)
            .finish()
    }
}

impl Display for OptionThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", OPTION_UNWRAP_MSG)?;
        write!(f, "At {}", self.location)
    }
}

impl Error for OptionThrow {}
