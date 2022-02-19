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
        let mut word = Word::new("something", Point2 { x, y }, 10.0, mock_sprite, false).unwrap();

        let old_pos = word.pos.clone();
        word.update(10.0);

        word.pos.x > old_pos.x && word.pos.y == old_pos.y
    }

    fn word_get_label(label: String) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0});
        let point = Point2 {
            x: 0.0,
            y: 0.0
        };
        let word = Word::new(&label, point, 10.0, mock_sprite, false).unwrap();

        word.label() == &label
    }

    fn word_translate(x: f32, y: f32) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0});
        let point = Point2 {
            x: 0.0,
            y: 0.0
        };
        let mut word = Word::new("test", point, 10.0, mock_sprite, false).unwrap();
        let old_pos = word.pos;
        word.translate(Point2 { x, y });

        ((x == 0.0 && old_pos.x == word.pos.x) ||
        (x != 0.0 && old_pos.x != word.pos.x)) &&
        ((y == 0.0 && old_pos.y == word.pos.y) ||
        (y != 0.0 && old_pos.y != word.pos.y))
    }

    fn word_reset_translation(x: f32, y: f32) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0});
        let point = Point2 {
            x: 0.0,
            y: 0.0
        };
        let mut word = Word::new("test", point, 10.0, mock_sprite, false).unwrap();
        let old_pos = word.pos;
        word.translate(Point2 { x, y });
        word.reset_translation();

        old_pos.x == word.pos.x && old_pos.y == word.pos.y
    }

    fn word_get_reward(speed: f32, color_changing: bool, label:String) -> bool {
        let mock_sprite = Box::new(MockSprite { width: 100.0, height: 100.0});
        let point = Point2 {
            x: 0.0,
            y: 0.0
        };
        let mut word = Word::new(&label, point, speed, mock_sprite, color_changing).unwrap();
        let reward = word.get_reward();
        let color_multiplayer = {
            if color_changing {
                2.0;
            }

            1.0
        };

        let expected_reward = speed * color_multiplayer * (label.len() as f32) / 100.0;


      (reward - expected_reward).abs() < f32::EPSILON
    }
}