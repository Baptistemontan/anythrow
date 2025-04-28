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
