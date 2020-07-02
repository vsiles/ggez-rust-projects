use rand;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::path;

use cgmath;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::*;

mod paddle;
use paddle::Paddle;
mod ball;
use ball::Ball;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const PADDLE_SPEED: f32 = 200.0;

struct Fonts {
    font: ggez::graphics::Font,
    small: f32,
    large: f32,
    score: f32,
}

struct Sounds {
    paddle_hit: audio::Source,
    wall_hit: audio::Source,
    score: audio::Source,
}

enum GameState {
    Start,
    Serve,
    Play,
    Done,
}

struct State {
    state: GameState,
    player1_score: u32,
    player2_score: u32,
    winning_player: u32,
    serving_player: u32,
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    rng: ThreadRng,
    fonts: Fonts,
    sounds: Sounds,
}

impl State {
    fn new(ctx: &mut Context, rng: ThreadRng) -> GameResult<State> {
        let player1 = Paddle::new(30.0, 90.0, 15.0, 59.0);
        let player2 = Paddle::new(WIDTH - 30.0, HEIGHT - 90.0, 15.0, 59.0);
        let ball = Ball::new();

        let small = 24.0;
        let font = ggez::graphics::Font::new(ctx, "/fonts/pong_font.ttf")?;

        let sounds = Sounds {
            paddle_hit: audio::Source::new(ctx, "/sounds/paddle_hit.wav")?,
            wall_hit: audio::Source::new(ctx, "/sounds/wall_hit.wav")?,
            score: audio::Source::new(ctx, "/sounds/score.wav")?,
        };

        let state = State {
            state: GameState::Start,
            player1_score: 0,
            player2_score: 0,
            winning_player: 0,
            serving_player: 1,
            player1,
            player2,
            ball,
            rng,
            fonts: Fonts {
                font,
                small,
                large: 48.0,
                score: 72.0,
            },
            sounds,
        };
        Ok(state)
    }

    fn display_score(&mut self, ctx: &mut Context) -> GameResult<()> {
        let score1 = format!("{}", self.player1_score);
        let text1 = ggez::graphics::Text::new((score1, self.fonts.font, self.fonts.score));
        let pos1 = cgmath::Point2::new(WIDTH / 2.0 - 148.0, HEIGHT / 3.0);
        ggez::graphics::draw(ctx, &text1, (pos1,))?;

        let score2 = format!("{}", self.player2_score);
        let text2 = ggez::graphics::Text::new((score2, self.fonts.font, self.fonts.score));
        let pos2 = cgmath::Point2::new(WIDTH / 2.0 + 89.0, HEIGHT / 3.0);
        ggez::graphics::draw(ctx, &text2, (pos2,))
    }

    fn display_fps(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0);
        let fps = format!("FPS: {:.2}", timer::fps(ctx));
        let text = ggez::graphics::Text::new((fps, self.fonts.font, self.fonts.small));
        let pos = cgmath::Point2::new(30.0, 30.0);
        ggez::graphics::draw(ctx, &text, (pos, color))
    }
}

fn center(text_len: f32, font_size: f32) -> f32 {
    let middle = WIDTH / 2.0;
    let total_len = text_len * font_size;
    middle - (total_len / 4.0)
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::delta(ctx).as_secs_f32();
        // println!("dt {}", dt);

        match self.state {
            GameState::Serve => {
                self.ball.delta.y = self.rng.gen_range(-50.0, 50.0);
                if self.serving_player == 1 {
                    self.ball.delta.x = self.rng.gen_range(140.0, 200.0);
                } else {
                    self.ball.delta.x = -self.rng.gen_range(140.0, 200.0);
                }
            }
            GameState::Play => {
                if self.ball.collides(&self.player1) {
                    self.ball.delta.x = -self.ball.delta.x * 1.03;
                    self.ball.xy.x = self.player1.xy.x + 15.0;

                    if self.ball.delta.y < 0.0 {
                        self.ball.delta.y = -self.rng.gen_range(10.0, 150.0)
                    } else {
                        self.ball.delta.y = self.rng.gen_range(10.0, 150.0)
                    }
                    self.sounds.paddle_hit.play()?
                }

                if self.ball.collides(&self.player2) {
                    self.ball.delta.x = -self.ball.delta.x * 1.03;
                    self.ball.xy.x = self.player2.xy.x - 12.0;

                    if self.ball.delta.y < 0.0 {
                        self.ball.delta.y = -self.rng.gen_range(10.0, 150.0)
                    } else {
                        self.ball.delta.y = self.rng.gen_range(10.0, 150.0)
                    }
                    self.sounds.paddle_hit.play()?
                }

                if self.ball.xy.y <= 0.0 {
                    self.ball.xy.y = 0.0;
                    self.ball.delta.y = -self.ball.delta.y;
                    self.sounds.wall_hit.play()?
                }

                if self.ball.xy.y >= HEIGHT - 12.0 {
                    // BALL SIZE
                    self.ball.xy.y = HEIGHT - 12.0;
                    self.ball.delta.y = -self.ball.delta.y;
                    self.sounds.wall_hit.play()?
                }

                if self.ball.xy.x < 0.0 {
                    self.serving_player = 1;
                    self.player2_score = self.player2_score + 1;
                    self.sounds.score.play()?;

                    if self.player2_score == 10 {
                        self.winning_player = 2;
                        self.state = GameState::Done
                    } else {
                        self.state = GameState::Serve;
                        self.ball.reset()
                    }
                }

                if self.ball.xy.x > WIDTH {
                    self.serving_player = 2;
                    self.player1_score = self.player1_score + 1;
                    self.sounds.score.play()?;

                    if self.player1_score == 10 {
                        self.winning_player = 1;
                        self.state = GameState::Done
                    } else {
                        self.state = GameState::Serve;
                        self.ball.reset()
                    }
                }
            }
            _ => {}
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player1.dy = -PADDLE_SPEED
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player1.dy = PADDLE_SPEED
        } else {
            self.player1.dy = 0.0
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player2.dy = -PADDLE_SPEED
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player2.dy = PADDLE_SPEED
        } else {
            self.player2.dy = 0.0
        }

        match self.state {
            GameState::Play => self.ball.update(dt),
            _ => (),
        }

        self.player1.update(dt);
        self.player2.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, [40.0 / 255.0, 45.0 / 255.0, 52.0 / 255.0, 1.0].into());

        // UI
        match self.state {
            GameState::Start => {
                let text0 = ggez::graphics::Text::new((
                    "Welcome to Pong!",
                    self.fonts.font,
                    self.fonts.small,
                ));
                let text1 = ggez::graphics::Text::new((
                    "Press Enter to begin!",
                    self.fonts.font,
                    self.fonts.small,
                ));
                let x0 = center(16.0, self.fonts.small);
                let pos0 = cgmath::Point2::new(x0, 15.0);
                let x1 = center(21.0, self.fonts.small);
                let pos1 = cgmath::Point2::new(x1, 40.0);
                ggez::graphics::draw(ctx, &text0, (pos0,))?;
                ggez::graphics::draw(ctx, &text1, (pos1,))?;
            }
            GameState::Serve => {
                let msg = format!("Player {}'s serve!", self.serving_player);
                let len = msg.len();
                let text = ggez::graphics::Text::new((msg, self.fonts.font, self.fonts.small));
                let x = center(len as f32, self.fonts.small);
                let pos = cgmath::Point2::new(x, 40.0);
                ggez::graphics::draw(ctx, &text, (pos,))?;
            }
            GameState::Done => {
                let msg = format!("Player {} wins!", self.winning_player);
                let len = msg.len();
                let text = ggez::graphics::Text::new((msg, self.fonts.font, self.fonts.large));
                let x = center(len as f32, self.fonts.large);
                let pos = cgmath::Point2::new(x, 15.0);
                ggez::graphics::draw(ctx, &text, (pos,))?;

                let msg1 = "Press Enter to restart!";
                let len1 = msg1.len();
                let text1 = ggez::graphics::Text::new((msg1, self.fonts.font, self.fonts.small));
                let x1 = center(len1 as f32, self.fonts.small);
                let pos1 = cgmath::Point2::new(x1, 60.0);
                ggez::graphics::draw(ctx, &text1, (pos1,))?;
            }
            _ => (),
        }

        self.display_score(ctx)?;

        self.player1.render(ctx)?;
        self.player2.render(ctx)?;
        self.ball.render(ctx)?;

        self.display_fps(ctx)?;

        ggez::graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::Return => match self.state {
                GameState::Start => self.state = GameState::Serve,
                GameState::Serve => self.state = GameState::Play,
                GameState::Done => {
                    self.state = GameState::Serve;
                    self.ball.reset();
                    self.player1_score = 0;
                    self.player2_score = 0;

                    if self.winning_player == 1 {
                        self.serving_player = 2
                    } else {
                        self.serving_player = 1
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let resource_dir = path::PathBuf::from("./resources");
    println!("Resource dir = {}", resource_dir.display());
    let mut c = conf::Conf::new();
    c.window_mode.width = WIDTH;
    c.window_mode.height = HEIGHT;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "vinz")
        .conf(c)
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Pong Test")
                .vsync(false),
        )
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
    let rng = rand::thread_rng();

    let state = &mut State::new(ctx, rng).unwrap();

    event::run(ctx, event_loop, state).unwrap();
    println!("Ggez exited")
}
