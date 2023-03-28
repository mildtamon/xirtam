mod integrate;
mod matrix_det;
mod matrix_inverse;
mod matrix_multiply_dot;
mod matrix_trace;
mod matrix_transpose;
use std::time::{Duration, Instant};

fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
}

fn main() {
    println!("Hello, world!");
}
