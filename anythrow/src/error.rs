use std::error::Error;
use std::fmt::{Debug, Display};
use std::panic::Location;

pub(crate) struct OptionThrowNone {
    _marker: (),
    location: &'static Location<'static>,
}

impl OptionThrowNone {
    pub const fn new() -> Self {
        OptionThrowNone {
            _marker: (),
            location: Location::caller(),
        }
    }
}

const OPTION_UNWRAP_MSG: &str = "called `Option::unwrap()` on a `None` value";

impl Debug for OptionThrowNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionThrow")
            .field("msg", &OPTION_UNWRAP_MSG)
            .field("location", self.location)
            .finish()
    }
}

impl Display for OptionThrowNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", OPTION_UNWRAP_MSG)?;
        write!(f, "At {}", self.location)
    }
}

impl Error for OptionThrowNone {}
