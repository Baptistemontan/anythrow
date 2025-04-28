use std::{
    any::Any,
    cell::RefCell,
    panic::{PanicHookInfo, UnwindSafe, catch_unwind},
};

// use crate::dbg;
use crate::{ResultLike, error::OptionThrowNone};

type PanickHook = Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync + 'static>;

thread_local! {
    static OLD_PANIC_HOOK: RefCell<Option<PanickHook>> = const { RefCell::new(None) };
}

pub fn try_catch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
    F: UnwindSafe,
    R: ResultLike,
{
    #[cfg(any(debug_assertions, feature = "debug"))]
    let _guard = crate::dbg::TryCatchContextGuard::new::<R>();
    match catch_unwind(f) {
        Ok(v) => v,
        Err(err) => {
            if let Some(old_panic_hook) = OLD_PANIC_HOOK.take() {
                std::panic::set_hook(old_panic_hook);
            }
            R::catch_or_resume(err)
        }
    }
}

#[cold]
#[track_caller]
fn start_throw<T: Any + Send>() {
    fn dummy_hook(_: &PanicHookInfo<'_>) {}

    #[cfg(any(debug_assertions, feature = "debug"))]
    crate::dbg::check_tid::<T>();

    let old_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(dummy_hook));
    let _ = OLD_PANIC_HOOK.replace(Some(old_panic_hook));
}

/// Throw the error
#[inline(always)]
#[track_caller]
pub fn throw<T: Any + Send>(err: T) -> ! {
    start_throw::<T>();
    std::panic::panic_any(err)
}

/// Thow a special catchable error that signal a `None` value.
#[inline(always)]
#[track_caller]
pub fn throw_none() -> ! {
    throw(OptionThrowNone::new())
}
