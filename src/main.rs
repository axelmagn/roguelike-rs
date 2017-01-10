extern crate piston_window;

use piston_window::{OpenGL, PistonWindow, WindowSettings, clear};

fn main() {
    let (width, height) = (800, 600);

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("roguelike", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 1.0, 1.0], g);
        });
    }
}
