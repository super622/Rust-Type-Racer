use ggez::conf::{ Conf, WindowMode };
use ggez::{ event, timer, filesystem };
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
use ggez::input::keyboard::is_key_pressed;
use ggez::mint::Point2;
use rand::Rng;
use rand::rngs::ThreadRng;

use type_racer::assets::TextSprite;
use type_racer::entities::Word;
use type_racer::debug;

use std::str;
use std::env;
use std::path;
use std::io::Read;

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
    current_input: String,
    score: u32,
    remaining_lifes: u32,
    words: Vec<Word>,
    time_until_next_word: f32,
    screen_width: f32,
    screen_height: f32,
    words_pool: Vec<String>
}

impl MainState {
    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {

        let file = filesystem::open(ctx, "/words.dict");
        
        if file.is_err() {
            panic!("Missing words dictionary!");
        }

        let mut buffer = Vec::new();
        let read_size = file?.read_to_end(&mut buffer);

        if read_size.is_err() || read_size? == 0 {
            panic!("Empty file with words dictionary!");
        }

        let words = str::from_utf8(&buffer).unwrap().split('\n').collect::<Vec<&str>>();
        let words = words.iter().map(|x| x.to_string());

        let start_state = MainState {
            rng: rand::thread_rng(),
            game_over: false,
            current_input: String::new(),
            score: 0,
            remaining_lifes: 5,
            words: Vec::new(),
            time_until_next_word: 4.0,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            words_pool: words.collect()
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
            
                let random_word = self.words_pool[self.rng.gen_range(0 .. self.words_pool.len())].clone();
                let random_speed = self.rng.gen_range(50.0 .. 200.0);
    
                let word_sprite = Box::new(TextSprite::new(&random_word, ctx)?);
                let word = Word::new(&random_word, random_point, random_speed, word_sprite)?;
    
                self.words.push(word);
                self.time_until_next_word = self.rng.gen_range(2.5 .. 4.8);
            }

            for word in self.words.iter_mut() {
                word.update(seconds);
    
                if word.label() == self.current_input {
                    word.is_typed = true;
                    self.score += 10;
                    // clear the input field after successfully typing word
                    self.current_input = String::new();
                }

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
            event::KeyCode::A => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "a", "A")
            },
            event::KeyCode::B => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "b", "B")
            },
            event::KeyCode::C => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "c", "C")
            },
            event::KeyCode::D => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "d", "D")
            },
            event::KeyCode::E => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "e", "E")
            },
            event::KeyCode::F => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "f", "F")
            },
            event::KeyCode::G => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "g", "G")
            },
            event::KeyCode::H => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "h", "H")
            },
            event::KeyCode::I => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "i", "I")
            },
            event::KeyCode::J => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "j", "J")
            },
            event::KeyCode::K => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "k", "K")
            },
            event::KeyCode::L => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "l", "L")
            },
            event::KeyCode::M => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "m", "M")
            },
            event::KeyCode::N => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "n", "N")
            },
            event::KeyCode::O => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "o", "O")
            },
            event::KeyCode::P => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "p", "P")
            },
            event::KeyCode::Q => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "q", "Q")
            },
            event::KeyCode::R => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "r", "R")
            },
            event::KeyCode::S => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "s", "S")
            },
            event::KeyCode::T => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "t", "T")
            },
            event::KeyCode::U => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "u", "U")
            },
            event::KeyCode::V => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "v", "V")
            },
            event::KeyCode::W => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "w", "W")
            },
            event::KeyCode::X => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "x", "X")
            },
            event::KeyCode::Y => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "y", "Y")
            },
            event::KeyCode::Z => {
                self.current_input = check_shift_pressed(self.current_input.clone(), ctx, "z", "Z")
            },
            event::KeyCode::Back => {
                self.current_input.pop();
            }
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let background_color = graphics::Color::from_rgb(0, 0, 0);
        graphics::clear(ctx, background_color);

        if self.game_over {

            return Ok(())
        }

        let label_margin = 10.0;

        // Draw current user input
        let font = graphics::Font::new(ctx, "/RedHatDisplay-Regular.otf")?;
        let mut current_input = graphics::Text::new(format!("Input: {}", self.current_input));
        current_input.set_font(font, graphics::PxScale::from(40.0));

        let bottom_left = Point2 {
            x: 0.0,
            y: (self.screen_height - current_input.height(ctx))
        };
        graphics::draw(ctx, &current_input, graphics::DrawParam::default().dest(bottom_left))?;

        // Draw current score
        let mut score_label = graphics::Text::new(format!("Score: {}", self.score));
        score_label.set_font(font, graphics::PxScale::from(40.0));

        let bottom_right = Point2 {
            x: (self.screen_width - score_label.width(ctx) - label_margin),
            y: (self.screen_height - score_label.height(ctx))
        };
        graphics::draw(ctx, &score_label, graphics::DrawParam::default().dest(bottom_right))?;

        // Draw remaining lifes
        let mut lifes_label = graphics::Text::new(format!("Lifes: {}", self.remaining_lifes));
        lifes_label.set_font(font, graphics::PxScale::from(40.0));

        let next_to_score = Point2 {
            x: (self.screen_width - score_label.width(ctx) - lifes_label.width(ctx) - label_margin * 2.0),
            y: (self.screen_height - lifes_label.height(ctx))
        };
        graphics::draw(ctx, &lifes_label, graphics::DrawParam::default().dest(next_to_score))?;

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

fn check_shift_pressed(current_input: String, ctx: &mut Context, lower_letter: &str, upper_letter: &str) -> String {
    if is_key_pressed(ctx, event::KeyCode::LShift) ||
       is_key_pressed(ctx, event::KeyCode::RShift) {
        return current_input + upper_letter;
    }

    current_input + lower_letter
}