use ggez::mint::Point2;
use ggez::{ Context, GameResult };
use ggez::graphics::Color;
use quickcheck::quickcheck;

use type_racer::entities::*;
use type_racer::assets::Sprite;

#[derive(Debug)]
struct MockSprite {
    width: f32,
    height: f32
}

impl Sprite for MockSprite {
    fn draw(&mut self, _top_left: Point2<f32>, _color: Color, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn width(&self, _ctx: &mut Context) -> f32 {
        self.width
    }

    fn height(&self, _ctx: &mut Context) -> f32 {
        self.height
    }
}

quickcheck! {
    fn words_move_left(x: f32, y: f32) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0});
        let mut word = Word::new("something", Point2 { x, y}, 10.0, mock_sprite, false).unwrap();

        let old_pos = word.pos.clone();
        word.update(10.0);

        word.pos.x > old_pos.x && word.pos.y == old_pos.y
    }
}