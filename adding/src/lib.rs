pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y: i32| x + y
}