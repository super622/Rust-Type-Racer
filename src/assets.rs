use ggez::audio::{self, SoundSource};
use ggez::{graphics, GameResult, Context};
use ggez::graphics::Color;
use ggez::mint::Point2;
use std::fmt::Debug;

pub struct Assets {
    pub word_typed_sound: audio::Source,
    pub background_music: audio::Source
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let word_typed_sound = audio::Source::new(ctx, "/collect-point.wav")?;
        let mut background_music = audio::Source::new(ctx, "/game-background-music.wav")?;
        background_music.set_repeat(true);

        Ok(Assets{
            word_typed_sound,
            background_music
        })
    }
}

pub trait Sprite: Debug {
    fn draw(&mut self, top_left: Point2<f32>, color: Color, ctx: &mut Context) -> GameResult<()>;
    fn width(&self, ctx: &mut Context) -> f32;
    fn height(&self, ctx: &mut Context) -> f32;
}

#[derive(Debug)]
pub struct TextSprite {
    text: graphics::Text
}

impl TextSprite {
    pub fn new(label: &str, ctx: &mut Context) -> GameResult<TextSprite> {
        let font = graphics::Font::new(ctx, "/RedHatDisplay-Regular.otf")?;
        let mut text = graphics::Text::new(label);
        text.set_font(font, graphics::PxScale::from(32.0));
        Ok(TextSprite { text })
    }
}

impl Sprite for TextSprite {
    fn draw(&mut self, top_left: Point2<f32>, color: Color, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.text, graphics::DrawParam::default().dest(top_left).color(color))
    }

    fn width(&self, ctx: &mut Context) -> f32 {
        self.text.width(ctx)
    }

    fn height(&self, ctx: &mut Context) -> f32 {
        self.text.height(ctx)
    }
}