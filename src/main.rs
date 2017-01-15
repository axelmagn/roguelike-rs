extern crate piston_window;
extern crate ai_behavior;
extern crate image;
extern crate find_folder;
extern crate graphics;
extern crate gfx_graphics;
extern crate glutin_window;
extern crate gfx_device_gl;
extern crate gfx_core;

mod drawable;
mod sprite;

use drawable::Drawable;
use graphics::image::Image;
use graphics::rectangle::square;
use image::{GenericImage};
use piston_window::{
    Button,
    Event,
    Flip,
    Input,
    Key,
    PistonWindow,
    Texture,
    TextureSettings,
    WindowSettings,
    clear,
};
use sprite::{Sprite, SpriteParams};
use std::rc::Rc;


fn main() {
    let (width, height) = (800, 600);

    // load image asset
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    // set up the window
    let mut window: PistonWindow =
        WindowSettings::new("roguelike", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let char_spritesheet = Texture::from_path(
            &mut window.factory,
            assets.join("roguelikeChar_transparent.png"),
            Flip::None,
            &TextureSettings::new())
        .unwrap();
    let char_spritesheet = Rc::new(char_spritesheet);

    let mut char_sprite = Sprite::from(
            char_spritesheet.clone(),
            SpriteParams::new()
                .src_rect([0.0, 5.0 * 17.0, 16.0, 16.0])
                .rect([16.0 * 10.0, 16.0 * 10.0, 16.0, 16.0]),
        );

    let up_arrow    = Event::Input(Input::Press(Button::Keyboard(Key::Up)));
    let down_arrow  = Event::Input(Input::Press(Button::Keyboard(Key::Down)));
    let left_arrow  = Event::Input(Input::Press(Button::Keyboard(Key::Left)));
    let right_arrow = Event::Input(Input::Press(Button::Keyboard(Key::Right)));

    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 1.0, 1.0], g);
                    char_sprite.draw(c, g);
                });
            }
            _ => {}
        }
    }
}
