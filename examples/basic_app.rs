//!
//! Basic App Example
//!

extern crate sunlite;

fn main() {
    let mut app = sunlite::AppBuilder::new()
        .title("Sunlite App")
        .size(800, 600)
        .min_size(800, 600)
        .max_size(1200, 800)
        .build();

    while let Some(_) = app.run() {
        //
    }
}
