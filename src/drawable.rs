use graphics::Graphics;
use graphics::context::Context;

pub trait Drawable<G: Graphics> {
    fn draw(&self, Context, &mut G);
}
