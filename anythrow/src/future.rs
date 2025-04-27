use std::{
    panic::{AssertUnwindSafe, UnwindSafe, catch_unwind},
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;

use crate::ResultLike;

#[pin_project]
#[derive(Clone, Copy)]
pub struct TryCatchFut<F: ?Sized>(#[pin] F);

impl<F> Future for TryCatchFut<F>
where
    F: Future,
    F::Output: ResultLike,
    F: UnwindSafe,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = catch_unwind(AssertUnwindSafe(move || this.0.poll(cx)));
        match res {
            Ok(v) => v,
            Err(err) => Poll::Ready(<F::Output as ResultLike>::catch_or_resume(err)),
        }
    }
}

/// Wrap the future with a try/catch mechanism,
/// correctly handling the error path in an async context.
pub fn try_catch_fut<F>(f: F) -> TryCatchFut<F>
where
    F: Future,
    F::Output: ResultLike,
    F: UnwindSafe,
{
    TryCatchFut(f)
}
