extern crate piston_window;
extern crate ai_behavior;
extern crate image;
extern crate find_folder;
extern crate sprite;

use std::rc::Rc;
use piston_window::{
    OpenGL,
    PistonWindow,
    WindowSettings,
    clear,
    Texture,
    TextureSettings,
    Event,
    Input,
    Key,
    Button,
};
use image::GenericImage;
use sprite::{
    Scene,
    Sprite,
    Ease,
    EaseFunction,
    MoveBy,
};
use ai_behavior::Action;

fn main() {
    let (width, height) = (800, 600);

    // load image asset
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut spritesheet = image::open(assets.join("roguelikeChar_transparent.png"))
        .unwrap()
        .to_rgba();

    // set up the window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("roguelike", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    // set up character sprite
    let char_img = {
        let tile_size = 16;
        let margin = 1;
        let (x, y, w, h) = (0, 6 * (tile_size + margin), tile_size, tile_size);
        spritesheet.sub_image(x, y, w, h).to_image()
    };
    let char_tex = Rc::new(Texture::from_image(
            &mut window.factory,
            &char_img,
            &TextureSettings::new()
        ).unwrap());
    let mut char_sprite = Sprite::from_texture(char_tex.clone());
    char_sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);

    // set up scene
    let mut scene = Scene::new();
    let id = scene.add_child(char_sprite);


    while let Some(e) = window.next() {
        scene.event(&e);

        match e {
            Event::Input(Input::Press(Button::Keyboard(Key::Up))) => {
                let (dx, dy) = (0.0, -16.0);
                let move_dir = Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, dx, dy))));
                scene.run(id, &move_dir);
            }

            Event::Input(Input::Press(Button::Keyboard(Key::Down))) => {
                let (dx, dy) = (0.0, 16.0);
                let move_dir = Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, dx, dy))));
                scene.run(id, &move_dir);
            }

            Event::Input(Input::Press(Button::Keyboard(Key::Left))) => {
                let (dx, dy) = (-16.0, 0.0);
                let move_dir = Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, dx, dy))));
                scene.run(id, &move_dir);
            }

            Event::Input(Input::Press(Button::Keyboard(Key::Right))) => {
                let (dx, dy) = (16.0, 0.0);
                let move_dir = Action(Ease(EaseFunction::CubicOut, Box::new(MoveBy(1.0, dx, dy))));
                scene.run(id, &move_dir);
            }

            Event::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 1.0, 1.0], g);
                    scene.draw(c.transform, g);
                });
            }
            _ => {}
        }
    }
}
