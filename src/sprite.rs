use drawable::Drawable;
use graphics::math::{Scalar, Matrix2d, identity};
use graphics::types::{Rectangle, SourceRectangle, Color};
use graphics::{self, Graphics, Context, ImageSize};
use std::rc::Rc;


/// Parameters used to specify how a sprite should be drawn
pub type SpriteParams = graphics::image::Image;


/// A Sprite contains an image as well as instructions on how to draw it in 2D.
///
/// Sprites do not contain any sub-images.
#[derive(Clone)]
pub struct Sprite<T> {
    /// source texture
    pub texture: Rc<T>,

    /// Color mask
    pub color: Color,
    /// Clipping rectangle over the texture
    pub src_rect: SourceRectangle,
    /// Sprite transform within its context
    pub transform: Matrix2d,
}


impl<T: ImageSize> Sprite<T> {
    /// Create a sprite from a texture, using default parameters.
    pub fn from_texture(texture: Rc<T>) -> Self {
        Self::from(texture, SpriteParams::new())
    }

    /// Create a sprite from a texture and a set of SpriteParams.
    ///
    /// If color or rectangles are omitted in params, default_color, default_src_rect, and
    /// default_tgt_rect are used to select default values.
    pub fn from(texture: Rc<T>, params: SpriteParams) -> Self {
        let color = params.color.unwrap_or(Self::default_color());
        let default_src_rect    = {
            let tex_ref: &T = &texture;
            Self::default_src_rect(tex_ref)
        };
        let src_rect        = params.source_rectangle.unwrap_or(default_src_rect);
        let default_tgt_rect = Self::default_tgt_rect(&src_rect);
        let tgt_rect        = params.rectangle.unwrap_or(default_tgt_rect);
        Sprite {
            texture:    texture,
            color:      color,
            src_rect:   src_rect,
            transform:  identity(),
        }
    }

    pub fn default_color() -> Color {
        [1.0, 1.0, 1.0, 1.0]
    }

    pub fn default_src_rect(texture: &T) -> SourceRectangle {
        [0.0, 0.0, texture.get_width() as Scalar, texture.get_height() as Scalar]
    }

    pub fn default_tgt_rect(src_rect: &SourceRectangle) -> Rectangle {
        [0.0, 0.0, src_rect[2], src_rect[3]]
    }

    pub fn sprite_params(&self) -> SpriteParams {
        SpriteParams {
            color: Some(self.color.clone()),
            source_rectangle: Some(self.src_rect.clone()),
            rectangle: Some(self.tgt_rect.clone()),
        }
    }
}


impl<G: Graphics> Drawable<G> for Sprite<G::Texture> {
    fn draw(&self, context: Context, graphics: &mut G) {
        let texture: &G::Texture = &self.texture;
        self.sprite_params().draw(
            texture,
            &context.draw_state,
            context.transform.append_transform(self.transform),
            graphics);
    }
}


impl<T> Positioned for Sprite<T> {
    fn pos(&self) -> Vec2d {
        return [
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    use graphics::rectangle::square;
    use piston_window::{
        PistonWindow,
        WindowSettings,
        Texture,
    };

    #[test]
    fn test_sprite_create() {
        let mut window: PistonWindow = WindowSettings::new("Test!", [512; 2]).build().unwrap();
        let texture = Rc::new(Texture::empty(&mut window.factory).unwrap());
        let draw_instrs = Image::new().src_rect(square(0.0, 0.0, 16.0));
        Sprite::from(texture, draw_instrs);
    }
}
