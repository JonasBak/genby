use cfg_if::cfg_if;
use rand::Rng;
use std::ops;
use vec;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn new_random_vec() -> vec::Vec2f {
    let mut rng = rand::thread_rng();
    vec::normalize(&vec::Vec2f::new(
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
    ))
}

pub fn interpolate(a: f32, b: f32, t: f32) -> f32 {
    let u = t * t * (3.0 - 2.0 * t);
    (1.0 - u) * a + u * b
}

pub fn map_range<T>(from: ops::Range<T>, to: ops::Range<T>, value: T) -> T
where
    T: PartialOrd<T>
        + ops::Sub<T, Output = T>
        + ops::Div<T, Output = T>
        + ops::Mul<T, Output = T>
        + ops::Add<T, Output = T>
        + Copy
        + Clone,
{
    (to.end - to.start) * (value - from.start) / (from.end - from.start) + to.start
}
