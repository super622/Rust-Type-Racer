use ggez::{ Context, GameResult, graphics };
use ggez::mint::{ Point2, Vector2 };

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::assets::Sprite;

#[derive(Debug)]
pub struct Word {
    pub pos: Point2<f32>,
    pub is_typed: bool,
    pub is_color_changing: bool,
    rng: ThreadRng,
    label: String,
    velocity: Vector2<f32>,
    sprite: Box<dyn Sprite>
}

impl Word {
    pub fn new(label: &str, pos: Point2<f32>, speed: f32, sprite: Box<dyn Sprite>, is_color_changing: bool) -> GameResult<Self> {
        let label = String::from(label);

        Ok(Word {
            pos,
            is_typed: false,
            is_color_changing,
            rng: rand::thread_rng(),
            label,
            velocity: Vector2 { x: speed, y: 0.0 },
            sprite
        })
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn update(&mut self, seconds: f32) {
        self.pos.x += self.velocity.x * seconds;
        self.pos.y += self.velocity.y * seconds;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.is_color_changing {
            self.sprite.draw(self.pos,
                       graphics::Color::from_rgb(
                                self.rng.gen_range(0 ..= 255),
                                self.rng.gen_range(0 ..= 255),
                                self.rng.gen_range(0 ..= 255)), ctx)
        }
        else {
            self.sprite.draw(self.pos, graphics::Color::from_rgb(255, 255, 255), ctx)
        }
    }

    // display sprite boundaries (for debug purposes)
    pub fn bounding_rect(&self, ctx: &mut Context) -> graphics::Rect {
        let left = self.pos.x;
        let right = self.pos.x + self.sprite.width(ctx);
        let top = self.pos.y;
        let bottom = self.pos.y + self.sprite.height(ctx);

        graphics::Rect::new(left, top, right - left, bottom - top)
    }
}