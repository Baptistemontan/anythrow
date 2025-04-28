use std::{
    any::Any,
    cell::RefCell,
    panic::{PanicHookInfo, UnwindSafe, catch_unwind},
};

mod error;
mod extensions;
mod future;
mod result_like;

pub use anythrow_macro::try_catch;
use error::OptionThrowNone;
pub use extensions::*;
pub use future::{TryCatchFut, try_catch_fut};
pub use result_like::ResultLike;

type PanickHook = Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync + 'static>;

#[cfg(any(debug_assertions, feature = "debug"))]
mod dbg {
    use crate::ResultLike;
    use std::{any::TypeId, cell::RefCell};

    thread_local! {
        pub static CONTEXT: RefCell<TryCatchContext> = const { RefCell::new(TryCatchContext::new()) };
    }

    pub struct Context {
        type_ids: Vec<TypeId>,
    }

    pub struct TryCatchContext {
        contexts: Vec<Context>,
    }

    impl Context {
        fn can_catch(&self, tid: TypeId) -> bool {
            self.type_ids.contains(&tid)
        }
    }

    impl TryCatchContext {
        pub fn can_catch(&self, tid: TypeId) -> Option<bool> {
            let can_catch = self.contexts.iter().rev().any(|ctx| ctx.can_catch(tid));
            if self.contexts.is_empty() {
                None
            } else {
                Some(can_catch)
            }
        }

        pub const fn new() -> Self {
            TryCatchContext {
                contexts: Vec::new(),
            }
        }

        pub fn push<R: ResultLike>(&mut self) {
            let ctx = Context {
                type_ids: R::catch_ids().into_iter().collect(),
            };

            self.contexts.push(ctx);
        }

        pub fn pop(&mut self) -> Option<Context> {
            self.contexts.pop()
        }
    }
}

thread_local! {
    static OLD_PANIC_HOOK: RefCell<Option<PanickHook>> = const { RefCell::new(None) };
}

pub fn try_catch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
    F: UnwindSafe,
    R: ResultLike,
{
    let _guard = TryCatchContextGuard::new::<R>();
    match catch_unwind(f) {
        Ok(v) => v,
        Err(err) => R::catch_or_resume(err),
    }
}

struct TryCatchContextGuard(());

impl Drop for TryCatchContextGuard {
    fn drop(&mut self) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        dbg::CONTEXT.with_borrow_mut(dbg::TryCatchContext::pop);
        if let Some(old_hook) = OLD_PANIC_HOOK.take() {
            std::panic::set_hook(old_hook);
        }
    }
}

impl TryCatchContextGuard {
    fn new<R: ResultLike>() -> Self {
        #[cfg(any(debug_assertions, feature = "debug"))]
        dbg::CONTEXT.with_borrow_mut(dbg::TryCatchContext::push::<R>);
        TryCatchContextGuard(())
    }
}

#[cold]
#[track_caller]
fn start_throw<T: Any + Send>() {
    fn dummy_hook(_: &PanicHookInfo<'_>) {}

    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        let tid = std::any::TypeId::of::<T>();
        let can_catch = dbg::CONTEXT.with_borrow(|ctx| ctx.can_catch(tid));
        match can_catch {
            None => todo!("No parent try catch."),
            Some(false) => todo!(
                "No parent try catch can catch value of type {:?}.",
                std::any::type_name::<T>()
            ),
            Some(true) => {}
        }
    }

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
