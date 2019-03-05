The [deriv1.rs](https://gist.github.com/tov/af73f345710e937ec39a4dbaca4504fe) program is a Rust port of my symbolic derivative benchmark by Jesse Tov that uses reference counting (Rust's `Rc`). This is a useful point for comparing the characteristics of reference counting and other memory management strategies such as tracing garbage collection.

The [deriv2.rs](https://github.com/TeXitoi/deriv-rs/blob/master/src/main.rs) program is a version optimised by Guillaume P. (@TeXitoi) to use an arena instead of reference counting. This is more representative of Rust's performance.
