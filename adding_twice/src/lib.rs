pub fn twice<F>(f: F) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32 + 'static,
{
    move |x| f(f(x))
}
pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y: i32| x + y
}