use ggez::conf::{ Conf, WindowMode };
use ggez::{ event, timer, filesystem };
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
use ggez::mint::Point2;
use rand::Rng;
use rand::rngs::ThreadRng;

use type_racer::assets::TextSprite;
use type_racer::entities::Word;
use type_racer::debug;

use std::env;
use std::path;

fn main() {
    let conf = Conf::new()
    .window_mode(WindowMode {
        width: 1200.0,
        height: 1000.0,
        ..Default::default()
    });

    let (mut ctx, event_loop) = ContextBuilder::new("type_racer", "George Shavov")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = MainState::new(&mut ctx, &conf).unwrap();

    event::run(ctx, event_loop, state);
}

struct MainState {
    rng: ThreadRng,
    game_over: bool,
    score: u32,
    remaining_lifes: u32,
    words: Vec<Word>,
    time_until_next_word: f32,
    screen_width: f32,
    screen_height: f32
}

impl MainState {
    const WORDS: [&'static str; 6] = [
        "Rust", "programming", "community",
        "rusty", "random", "facility"
    ];

    fn new(_ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {

        let start_state = MainState {
            rng: rand::thread_rng(),
            game_over: false,
            score: 0,
            remaining_lifes: 5,
            words: Vec::new(),
            time_until_next_word: 1.0,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height
        };

        Ok(start_state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(())
        }

        const FPS_CAP: u32 = 60;

        while timer::check_update_time(ctx, FPS_CAP)
        {
            let seconds = 1.0 / (FPS_CAP as f32);

            // Spawn  words
            self.time_until_next_word -= seconds;
            if self.time_until_next_word <= 0.0 {
                let random_point = Point2 {
                    x: 0.0,
                    //TODO: check if 100.0 is okey for word size
                    y: self.rng.gen_range(0.0 .. self.screen_height - 100.0)
                };
            
                let random_word = Self::WORDS[self.rng.gen_range(0 .. Self::WORDS.len())];
                let random_speed = self.rng.gen_range(50.0 .. 200.0);
    
                let word_sprite = Box::new(TextSprite::new(random_word, ctx)?);
                let word = Word::new(random_word, random_point, random_speed, word_sprite)?;
    
                self.words.push(word);
                self.time_until_next_word = self.rng.gen_range(0.5 .. 1.8);
            }

            for word in self.words.iter_mut() {
                word.update(seconds);
    
                if word.pos.x >= self.screen_width {
                    word.is_typed = true;

                    if !debug::is_active() {
                        // don't end the game is debug is active
                        self.remaining_lifes -= 1;

                        if self.remaining_lifes == 0 {
                            self.game_over = true;
                        }
                    }
                }
            }

            self.words.retain(|word| !word.is_typed);
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::Escape => event::quit(ctx),
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let background_color = graphics::Color::from_rgb(0, 0, 0);
        graphics::clear(ctx, background_color);

        if self.game_over {

            return Ok(())
        }

        for word in self.words.iter_mut() {
            word.draw(ctx)?;
        }

        if debug::is_active() {
            for word in &mut self.words {
                debug::draw_outline(word.bounding_rect(ctx), ctx).unwrap();
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}