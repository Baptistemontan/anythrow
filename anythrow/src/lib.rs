#[cfg(any(debug_assertions, feature = "debug"))]
mod dbg;

mod error;
mod extensions;
mod future;
mod imp;
mod result_like;

pub use anythrow_macro::try_catch;
pub use extensions::*;
pub use future::{TryCatchFut, try_catch_fut};
pub use imp::{throw, throw_none, try_catch};
pub use result_like::ResultLike;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_catch() {
        let expected_val = 45i32;
        let result = try_catch(|| -> Result<(), i32> { throw(expected_val) });
        let err = result.unwrap_err();

        assert_eq!(err, expected_val);
    }

    #[test]
    fn can_catch_none() {
        let result = try_catch(|| -> Option<()> { throw_none() });

        assert!(result.is_none())
    }

    #[test]
    fn multiple_depth() {
        let result = try_catch(|| -> Option<()> {
            let _ = try_catch(|| -> Result<(), ()> { throw_none() });
            Some(())
        });

        assert!(result.is_none());
    }
}
