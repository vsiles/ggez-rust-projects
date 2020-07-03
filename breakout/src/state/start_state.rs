use ggez;
use ggez::audio::SoundSource;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

use cgmath::Point2;

use super::super::Fonts;
use super::super::GlobalState;
use super::super::SoundKind;
use super::super::Sounds;
use super::StartState;
use super::State;
use super::StateKind;

use super::super::HEIGHT;
use super::super::WIDTH;

const TITLE: &str = "BREAKOUT";
const TITLE_LEN: f32 = TITLE.len() as f32;

const START: &str = "START";
const START_LEN: f32 = START.len() as f32;

const HIGH_SCORES: &str = "HIGH SCORES";
const HIGH_SCORES_LEN: f32 = HIGH_SCORES.len() as f32;

impl StartState {
    pub fn new(fonts: &Fonts) -> StartState {
        StartState {
            highlighted: 1,
            title: ggez::graphics::Text::new((TITLE, fonts.font, fonts.large)),
            start: ggez::graphics::Text::new((START, fonts.font, fonts.medium)),
            high_score: ggez::graphics::Text::new((HIGH_SCORES, fonts.font, fonts.medium)),
        }
    }
}

impl State for StartState {
    fn enter(&mut self, _params: StateKind) {}

    fn exit(&self) {}

    fn update(&mut self, sounds: &mut Sounds, ctx: &mut Context) -> GameResult<Option<StateKind>> {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.highlighted = 1;
            let key = &SoundKind::PaddleHit.to_string();
            sounds.get_mut(key).unwrap().play()?
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.highlighted = 2;
            let key = &SoundKind::PaddleHit.to_string();
            sounds.get_mut(key).unwrap().play()?
        }

        let next = if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            let key = &SoundKind::Confirm.to_string();
            sounds.get_mut(key).unwrap().play()?;
            if self.highlighted == 1 {
                Some(StateKind::PaddleSelect)
            } else {
                Some(StateKind::HighScores)
            }
        } else {
            None
        };

        if keyboard::is_key_pressed(ctx, KeyCode::Escape) {
            ggez::event::quit(ctx)
        }

        Ok(next)
    }

    fn render(&self, _global_state: &GlobalState, ctx: &mut Context) -> GameResult<()> {
        let title_shift = 15.0 * TITLE_LEN;
        let title_pos = Point2::new((WIDTH / 2.0) - title_shift, HEIGHT / 3.0);
        ggez::graphics::draw(ctx, &self.title, (title_pos,))?;

        let color = if self.highlighted == 1 {
            ggez::graphics::Color::new(103.0 / 255.0, 1.0, 1.0, 1.0)
        } else {
            ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0)
        };
        let start_shift = 10.0 * START_LEN;
        let start_pos = Point2::new((WIDTH / 2.0) - start_shift, (HEIGHT / 2.0) + 208.0);
        ggez::graphics::draw(ctx, &self.start, (start_pos, color))?;

        let color = if self.highlighted == 2 {
            ggez::graphics::Color::new(103.0 / 255.0, 1.0, 1.0, 1.0)
        } else {
            ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0)
        };
        let shift = 9.0 * HIGH_SCORES_LEN;
        let pos = Point2::new((WIDTH / 2.0) - shift, (HEIGHT / 2.0) + 267.0);
        ggez::graphics::draw(ctx, &self.high_score, (pos, color))
    }
}
