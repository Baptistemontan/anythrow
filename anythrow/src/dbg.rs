use crate::ResultLike;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
};

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
    pub fn can_catch(&self, tid: TypeId) -> bool {
        self.contexts.iter().rev().any(|ctx| ctx.can_catch(tid))
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

pub struct TryCatchContextGuard(());

impl Drop for TryCatchContextGuard {
    fn drop(&mut self) {
        CONTEXT.with_borrow_mut(TryCatchContext::pop);
    }
}

impl TryCatchContextGuard {
    pub fn new<R: ResultLike>() -> Self {
        CONTEXT.with_borrow_mut(TryCatchContext::push::<R>);
        TryCatchContextGuard(())
    }
}

#[track_caller]
pub fn check_tid<T: Any + Send>() {
    let tid = std::any::TypeId::of::<T>();
    let can_catch = CONTEXT.with_borrow(|ctx| ctx.can_catch(tid));
    if !can_catch {
        todo!(
            "No parent try/catch can catch value of type {:?}.",
            std::any::type_name::<T>()
        )
    }
}
