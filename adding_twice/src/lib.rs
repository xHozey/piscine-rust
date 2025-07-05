pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn twice<F: Fn(i32) -> i32>(func: F) -> impl Fn(i32) -> i32 {
    move |x| func(func(x))
}
