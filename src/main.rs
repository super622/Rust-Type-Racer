use ggez::audio::SoundSource;
use ggez::conf::{ Conf, WindowMode };
use ggez::{ event, timer, filesystem, graphics };
use ggez::{ Context, ContextBuilder, GameResult };
use ggez::input::keyboard::is_key_pressed;
use ggez::mint::Point2;
use rand::{ Rng, seq };
use rand::rngs::ThreadRng;

use type_racer::assets::{ Assets, TextSprite, Sprite };
use type_racer::entities::Word;
use type_racer::debug;

use std::mem::swap;
use std::str;
use std::env;
use std::path;
use std::io::{Read, Write};

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

    graphics::set_window_title(&ctx, "Type Racer");

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
    assets: Assets,
    info_panel: TextSprite,
    sound_volume: f32,
    show_info: bool,
    game_over: bool,
    saved_score: bool,
    current_input: String,
    cash: f32,
    score: f32,
    remaining_lifes: u32,
    words: Vec<Word>,
    time_until_next_word: f32,
    game_speed_up: f32,
    time_until_shake: f32,
    shake_screen: bool,
    shake_time: f32,
    screen_width: f32,
    screen_height: f32,
    words_pool: Vec<String>,
    scoreboard: Vec<String>,
    power_up_panels: Vec<TextSprite>
}

impl MainState {
    const BUY_LIFE_TAX: f32 = 300.0;
    const REMOVE_WORDS_TAX: f32 = 350.0;
    const SLOW_WORD_SPAWN_TAX: f32 = 1000.0;
    const REMOVE_WORDS_COUNT: usize = 2;
    const INITAL_SOUND_VOLUME: f32 = 0.05;
    const SOUND_VOLUME_STEP: f32 = 0.005;
    const SCOREBOARD_SIZE: usize = 10;
    const TOP_PANEL_TEXT_SIZE: f32 = 34.0;
    const BOT_PANEL_TEXT_SIZE: f32 = 40.0;
    const CENTER_PANEL_TEXT_SIZE: f32 = 40.0;
    const SHAKE_DURATION: f32 = 1.0;
    const SHAKE_MAGNITUDE: f32 = 3.0;

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let mut assets = Assets::new(ctx)?;
        assets.background_music.set_volume(MainState::INITAL_SOUND_VOLUME);
        let _ = assets.background_music.play(ctx);
        let words = read_file_by_lines(ctx, "/words.dict");

        let info_panel_label = format!(
"(+) to volume up
(-) to volume down

Buffs become visible when you have the required cash:
(1) for extra life  ({:.2}$)
(2) for words removal  ({:.2}$)
(3) for slow words spawn  ({:.2}$)

(Esc) to quit",
                           MainState::BUY_LIFE_TAX,
                           MainState::REMOVE_WORDS_TAX,
                           MainState::SLOW_WORD_SPAWN_TAX);
        let info_panel = TextSprite::new(&info_panel_label, ctx, MainState::CENTER_PANEL_TEXT_SIZE)?;

        let slow_word_spawn_label = format!("(3) Slow spawn ({:.2}$)", MainState::SLOW_WORD_SPAWN_TAX);
        let slow_word_spawn_panel = TextSprite::new(&slow_word_spawn_label, ctx, MainState::TOP_PANEL_TEXT_SIZE)?;

        let remove_words_label = format!("(2) Remove {} words ({:.2}$)",MainState::REMOVE_WORDS_COUNT , MainState::REMOVE_WORDS_TAX);
        let remove_words_panel = TextSprite::new(&remove_words_label, ctx, MainState::TOP_PANEL_TEXT_SIZE)?;

        let extra_life_label = format!("(1) extra life ({:.2}$)", MainState::BUY_LIFE_TAX);
        let extra_life_panel = TextSprite::new(&extra_life_label, ctx, MainState::TOP_PANEL_TEXT_SIZE)?;

        let mut power_up_panels = Vec::new();
        power_up_panels.push(slow_word_spawn_panel);
        power_up_panels.push(remove_words_panel);
        power_up_panels.push(extra_life_panel);

        let start_state = MainState {
            rng: rand::thread_rng(),
            assets: assets,
            info_panel,
            sound_volume: MainState::INITAL_SOUND_VOLUME,
            show_info: false,
            game_over: false,
            saved_score: false,
            current_input: String::new(),
            cash: 0.0,
            score: 0.0,
            remaining_lifes: 5,
            words: Vec::new(),
            time_until_next_word: 3.0,
            game_speed_up: 0.0,
            time_until_shake: 10.0,
            shake_screen: false,
            shake_time: MainState::SHAKE_DURATION,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            words_pool: words,
            scoreboard: Vec::new(),
            power_up_panels
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

            // Screen shaker
            self.time_until_shake -= seconds;
            if self.time_until_shake <= 0.0 {
                self.time_until_shake = self.rng.gen_range(5.0 .. 20.0);
                self.shake_screen = true;
            }

            if self.shake_screen {
                self.shake_time -= seconds;
                if self.shake_time <= 0.0 {
                    self.shake_time = MainState::SHAKE_DURATION;
                    self.shake_screen = false;
                }
            }

            // Spawn words
            self.time_until_next_word -= seconds;
            if self.time_until_next_word <= 0.0 {
                let margin = 10.0;
                let top_height = MainState::TOP_PANEL_TEXT_SIZE + margin;
                let bot_height = self.screen_height - MainState::BOT_PANEL_TEXT_SIZE - margin;
                let random_point = Point2 {
                    x: 0.0,
                    y: self.rng.gen_range(top_height .. bot_height)
                };
            
                let random_word = self.words_pool[self.rng.gen_range(0 .. self.words_pool.len())].clone();
                
                let random_speed = self.rng.gen_range(100.0 .. 300.0);
                let percentage: u8 = self.rng.gen_range(0 ..= 100);
                let is_color_changing = percentage < 30;
                let word_sprite = Box::new(TextSprite::new(&random_word, ctx, 32.0)?);
                let word = Word::new(&random_word, random_point, random_speed, word_sprite, is_color_changing)?;
    
                self.words.push(word);
                let min_word_gen_time = 3.0 - self.game_speed_up;
                let max_word_gen_time = 3.5 - self.game_speed_up;
                self.time_until_next_word = self.rng.gen_range(min_word_gen_time .. max_word_gen_time);
                self.game_speed_up += 0.03;
            }

            for word in self.words.iter_mut() {
                word.update(seconds);
    
                if word.label() == self.current_input {
                    word.is_typed = true;
                    
                    self.score += word.get_reward();
                    self.cash += word.get_reward();

                    self.assets.word_typed_sound.set_volume(self.sound_volume);
                    let _ = self.assets.word_typed_sound.play(ctx);

                    // clear the input field after successfully typed word
                    self.current_input = String::new();
                }

                if word.pos.x >= self.screen_width {
                    word.is_typed = true;

                    if !debug::is_active() {
                        // don't end the game when debug is active
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
            event::KeyCode::Key1 |
            event::KeyCode::Numpad1 => {
                if self.cash >= MainState::BUY_LIFE_TAX {
                    self.cash -= MainState::BUY_LIFE_TAX;
                    self.remaining_lifes += 1;
                }
            },
            event::KeyCode::Key2 |
            event::KeyCode::Numpad2 => {
                if self.cash >= MainState::REMOVE_WORDS_TAX && self.words.len() > 0 {
                    self.cash -= MainState::REMOVE_WORDS_TAX;

                    if self.words.len() <= MainState::REMOVE_WORDS_COUNT {
                        self.words.iter_mut().for_each(|word| {
                            word.is_typed = true;
                            self.score += word.get_reward();
                        });
                    }
                    else {
                        let sample_indexes = seq::index::sample(&mut self.rng, self.words.len(), MainState::REMOVE_WORDS_COUNT);

                        for index in sample_indexes.iter() {
                            self.words[index].is_typed = true;
                            self.score += self.words[index].get_reward();
                        }
                    }
                }
            },
            event::KeyCode::Key3 |
            event::KeyCode::Numpad3 => {
                if self.cash >= MainState::SLOW_WORD_SPAWN_TAX {
                    self.cash -= MainState::SLOW_WORD_SPAWN_TAX;
                    self.game_speed_up /= 2.0;
                }
            },
            event::KeyCode::NumpadAdd => {
                if self.sound_volume + MainState::SOUND_VOLUME_STEP <= 100.0 {
                    self.sound_volume += MainState::SOUND_VOLUME_STEP;
                    self.assets.background_music.set_volume(self.sound_volume);
                }
            },
            event::KeyCode::NumpadSubtract => {
                if self.sound_volume - MainState::SOUND_VOLUME_STEP >= 0.0 {
                    self.sound_volume -= MainState::SOUND_VOLUME_STEP;
                    self.assets.background_music.set_volume(self.sound_volume);
                }
            },
            event::KeyCode::Grave => {
                self.show_info ^= true;
            }
            event::KeyCode::Minus => {
                self.current_input += "-";
            },
            event::KeyCode::Return => {
                if !self.saved_score {
                    self.scoreboard = save_score(ctx, self.current_input.clone(), self.score);
                    self.current_input = String::new();
                    self.saved_score = true;
                }
            },
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
            },
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let background_color = graphics::Color::BLACK;
        graphics::clear(ctx, background_color);

        let label_margin = 10.0;
        let game_status_panel_color = graphics::Color::WHITE;
        let mut shake_translation: Point2<f32> = Point2 {
            x: 0.0,
            y: 0.0
        };

        if self.shake_screen {
            let dx = self.rng.gen_range(-MainState::SHAKE_MAGNITUDE ..=MainState::SHAKE_MAGNITUDE);
            let dy = self.rng.gen_range(-MainState::SHAKE_MAGNITUDE ..=MainState::SHAKE_MAGNITUDE);

            shake_translation.x = dx;
            shake_translation.y = dy;
        }

        // Draw current user input
        if !self.game_over || !self.saved_score {
            let mut bottom_left = Point2 {
                x: 0.0,
                y: self.screen_height
            };
    
            translate(&mut bottom_left, &shake_translation);

            let current_input_label = format!("Input: {}", self.current_input);
            let mut current_input_panel = TextSprite::new(&current_input_label, ctx, MainState::BOT_PANEL_TEXT_SIZE).unwrap();
            bottom_left.x += label_margin;
            bottom_left.y = self.screen_height - current_input_panel.height(ctx);
            current_input_panel.draw(bottom_left, game_status_panel_color, ctx).unwrap();
        }

        // Game over scene
        if self.game_over {

            if !self.saved_score {
                let ending;
                if self.score < 100.0 {
                    ending = "Bummer, I know you can do better :) Try again!";
                }
                else if self.score >= 100.0 && self.score < 500.0 {
                    ending = "Not very bad!";
                }
                else if self.score >= 500.0 && self.score < 1000.0 {
                    ending = "Amazing, but can you do better?"
                }
                else {
                    ending = "You're a madman, niiice :)"
                }

                let game_over_label = format!("Game over!\nYour score is : {:.2}\n{}\nType username for the scoreboard!", self.score, ending);
                let mut game_over_panel = TextSprite::new(&game_over_label, ctx, MainState::CENTER_PANEL_TEXT_SIZE).unwrap();

                let centered = Point2 {
                    x: (self.screen_width - game_over_panel.width(ctx)) / 2.0,
                    y: (self.screen_height - game_over_panel.height(ctx)) / 2.0
                };

                game_over_panel.draw(centered, game_status_panel_color, ctx).unwrap();
            }
            else {
                let scoreboard_label = format!("Scoreboard:\n{}", format_scoreboard(&self.scoreboard));
                let mut scoreboard_panel = TextSprite::new(&scoreboard_label, ctx, MainState::CENTER_PANEL_TEXT_SIZE).unwrap();

                let centered = Point2 {
                    x: (self.screen_width - scoreboard_panel.width(ctx)) / 2.0,
                    y: (self.screen_height - scoreboard_panel.height(ctx)) / 2.0
                };

                scoreboard_panel.draw(centered, game_status_panel_color, ctx).unwrap();
            }
            
            graphics::present(ctx)?;
            return Ok(())
        }

        // Game info panel
        if self.show_info {
            let centered = Point2 {
                x: (self.screen_width - self.info_panel.width(ctx)) / 2.0,
                y: (self.screen_height - self.info_panel.height(ctx)) / 2.0
            };

            let info_panel_color = graphics::Color::from_rgb(48, 116, 115);
            let silver = graphics::Color::from_rgb(192, 192, 192);

            draw_text_background(centered, self.info_panel.width(ctx), self.info_panel.height(ctx), 30.0, silver, ctx);
            self.info_panel.draw(centered, info_panel_color, ctx)?;
        }

        // Draw current volume
        let mut top_left = Point2 {
            x: 0.0,
            y: 0.0
        };

        translate(&mut top_left, &shake_translation);

        let options_label = format!("(`) for Info|");
        let mut options_panel = TextSprite::new(&options_label, ctx, MainState::TOP_PANEL_TEXT_SIZE).unwrap();
        top_left.x += label_margin;
        options_panel.draw(top_left, game_status_panel_color, ctx).unwrap();
        top_left.x += options_panel.width(ctx);

        let current_volume_label = format!("Volume: {:.0}", self.sound_volume * 100.0);
        let mut current_volume_panel = TextSprite::new(&current_volume_label, ctx, MainState::TOP_PANEL_TEXT_SIZE).unwrap();
        top_left.x += label_margin;
        current_volume_panel.draw(top_left, game_status_panel_color, ctx).unwrap();

        // Draw current cash
        let mut bottom_right = Point2 {
            x: self.screen_width,
            y: self.screen_height
        };

        translate(&mut bottom_right, &shake_translation);

        let cash_label = format!("Cash: {:.2}", self.cash);
        let mut cash_panel = TextSprite::new(&cash_label, ctx, MainState::BOT_PANEL_TEXT_SIZE).unwrap();
        bottom_right.x -= cash_panel.width(ctx) + label_margin;
        bottom_right.y -= cash_panel.height(ctx);
        cash_panel.draw(bottom_right, game_status_panel_color, ctx).unwrap();
        bottom_right.y += cash_panel.height(ctx);

        // Draw remaining lifes
        let lifes_label = format!("Lifes: {}", self.remaining_lifes);
        let mut lifes_panel = TextSprite::new(&lifes_label, ctx, MainState::BOT_PANEL_TEXT_SIZE).unwrap();
        bottom_right.x -= lifes_panel.width(ctx) + label_margin;
        bottom_right.y -= lifes_panel.height(ctx);
        lifes_panel.draw(bottom_right, game_status_panel_color, ctx).unwrap();
        bottom_right.y += lifes_panel.height(ctx);

        // Draw current score
        let score_label = format!("Score: {:.2}", self.score);
        let mut score_panel = TextSprite::new(&score_label, ctx, MainState::BOT_PANEL_TEXT_SIZE).unwrap();
        bottom_right.x -= score_panel.width(ctx) + label_margin;
        bottom_right.y -= score_panel.height(ctx);
        score_panel.draw(bottom_right, game_status_panel_color, ctx).unwrap();
        bottom_right.y += score_panel.height(ctx);

        // Draw power ups
        let power_up_color = graphics::Color::WHITE;
        let mut top_right = Point2 {
            x: self.screen_width,
            y: 0.0
        };

        translate(&mut top_right, &shake_translation);

        if self.cash >= MainState::SLOW_WORD_SPAWN_TAX {
            top_right.x -= self.power_up_panels[0].width(ctx) + label_margin;
            self.power_up_panels[0].draw(top_right, power_up_color, ctx).unwrap();
        }

        if self.cash >= MainState::REMOVE_WORDS_TAX {
            top_right.x -= self.power_up_panels[1].width(ctx) + label_margin;
            self.power_up_panels[1].draw(top_right, power_up_color, ctx).unwrap();
        }

        if self.cash >= MainState::BUY_LIFE_TAX {
            top_right.x -= self.power_up_panels[2].width(ctx) + label_margin;
            self.power_up_panels[2].draw(top_right, power_up_color, ctx).unwrap();
        }

        for word in self.words.iter_mut() {
            word.translate(shake_translation);

            if !self.shake_screen {
                word.reset_translation();
            }

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

fn read_file_by_lines(ctx: &Context, path: &str) -> Vec<String> {
    let file = filesystem::open(ctx, path);
        
    if file.is_err() {
        panic!("Error with opening {}!", path);
    }

    let mut buffer = Vec::new();
    let read_size = file.unwrap().read_to_end(&mut buffer);

    if read_size.is_err() || read_size.unwrap() == 0 {
        panic!("Empty file {}!", path);
    }

    let words = str::from_utf8(&buffer).unwrap().split('\n').collect::<Vec<&str>>();
    words.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn save_score(ctx: &Context, username: String, score: f32) -> Vec<String> {
    let mut file;
    if filesystem::exists(ctx, "/scoring.data") {
        let mut scores = read_file_by_lines(ctx, "/scoring.data");

        // check for empty line at the end
        if scores.len() > 0 && scores[scores.len() - 1] == "" {
            scores.pop();
        }

        let mut new_line = format!("{} {:.2}", username, score);
        let mut insert = false;
        for line in scores.iter_mut() {
            let split = line.split(" ").collect::<Vec<&str>>();
            let saved_score = split[split.len() - 1].parse::<f32>().unwrap();

            if saved_score < score
            {
                insert = true;
            }

            if insert {
                swap(line, &mut new_line);
            }
        }

        if scores.len() < MainState::SCOREBOARD_SIZE
        {
            scores.push(new_line);
        }

        file = filesystem::create(ctx, "/scoring.data").unwrap();

        let _ = file.write(scores.join("\n").as_bytes());

        return scores;
    }
    else {
        file = filesystem::create(ctx, "/scoring.data").unwrap();
    }

    let new_score = format!("{} {:.2}", username, score);
    let _ = file.write(new_score.as_bytes());
    let mut result = Vec::new();
    result.push(new_score);

    result
}

fn format_scoreboard(scoreboard: &Vec<String>) -> String {
    let mut result = String::new();

    for (index,score) in scoreboard.iter().enumerate()
    {
        let formatted = format!("{}) {}\n", index + 1, score);
        result.push_str(&formatted);
    }

    result
}

fn draw_text_background(text_pos: Point2<f32>, text_width: f32, text_height: f32, margin: f32, color: graphics::Color, ctx: &mut Context) {
    let left = text_pos.x - margin;
    let right = text_pos.x + text_width + margin;
    let top = text_pos.y - margin;
    let bottom = text_pos.y + text_height + margin;

    let background = graphics::Rect::new(left, top, right - left, bottom - top);
    let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT);
    let background_mesh = graphics::MeshBuilder::new().
        rectangle(draw_mode, background, color).
        unwrap().
        build(ctx).
        unwrap();

    graphics::draw(ctx, &background_mesh, graphics::DrawParam::default()).unwrap();
}

fn translate(pos: &mut Point2<f32>, trans: &Point2<f32>) {
    pos.x += trans.x;
    pos.y += trans.y;
}