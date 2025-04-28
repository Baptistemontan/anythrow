use anythrow::try_catch;

struct MyError(&'static str);

#[try_catch]
fn foo() -> Result<i32, MyError> {
    let val = bar();
    Ok(val)
}

fn bar() -> i32 {
    // anythrow::throw(MyError("test"))
    anythrow::throw("test")
}

fn main() {
    let result = foo();

    let err = result.unwrap_err();

    assert_eq!(err.0, "test");
}
