# Anythrow

Tired of explicit error handling ? With `?` everywhere ? Some `Result<CoolStuff, SuperBoringError>` being your n°1 return type ?

What if you could just throw you errors away and let **_some_** caller up the stack handle it ?

Try Anythrow !

With anythrow, only one function need to return a `Result<T, E>`, and any function called inside, at any depth, can throw a `E` error to be catched by the top level function !

```rust
use anythrow::try_catch;

struct MyError;

#[try_catch]
fn foo() -> Result<i32, MyError> {
    let val = bar();
    Ok(val)
}

fn bar() -> i32 {
    anythrow::throw(MyError)
}

fn main() {
    let result = foo();

    assert!(result.is_err())
}
```

Also works with `Option<T>`:

```rust
use anythrow::try_catch;

#[try_catch]
fn foo() -> Option<i32> {
    let val = bar();
    Some(val)
}

fn bar() -> i32 {
    anythrow::throw_none()
}

fn main() {
    let result = foo();

    assert!(result.is_none())
}
```

And of course, works with async and behave as expected:

```rust
use anythrow::try_catch;

#[try_catch]
async fn foo() -> Option<i32> {
    let val = bar().await;
    Some(val)
}

async fn bar() -> i32 {
    anythrow::throw_none()
}

fn main() {
    let result = block_on(foo());

    assert!(result.is_none())
}
```

## Unwinding

This crate use the unwind mechanism of panics to work, if you have `panic = "abort"` this is basically useless.

# Disclaimer

This is absolutely not idiomatic Rust, please don't use this library.
