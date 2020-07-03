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
use super::HighScoreState;
use super::State;
use super::StateKind;

use super::super::HEIGHT;
use super::super::WIDTH;

const TEXT: &str = "High Scores";
const TEXT_LEN: f32 = TEXT.len() as f32;

const EXIT: &str = "Press Escape to return to the main menu";
const EXIT_LEN: f32 = EXIT.len() as f32;

impl HighScoreState {
    pub fn new(fonts: &Fonts) -> HighScoreState {
        HighScoreState {
            text: ggez::graphics::Text::new((TEXT, fonts.font, fonts.large)),
            exit: ggez::graphics::Text::new((EXIT, fonts.font, fonts.small)),
        }
    }
}

impl State for HighScoreState {
    fn enter(&mut self, _params: StateKind) {}

    fn exit(&self) {}

    fn update(&mut self, sounds: &mut Sounds, ctx: &mut Context) -> GameResult<Option<StateKind>> {
        if keyboard::is_key_pressed(ctx, KeyCode::Escape) {
            let key = &SoundKind::WallHit.to_string();
            sounds.get_mut(key).unwrap().play()?;
            Ok(Some(StateKind::Start))
        } else {
            Ok(None)
        }
    }

    fn render(&self, global_state: &GlobalState, ctx: &mut Context) -> GameResult<()> {
        let shift = 15.0 * TEXT_LEN;
        let pos = Point2::new((WIDTH / 2.0) - shift, HEIGHT / 6.0);
        ggez::graphics::draw(ctx, &self.text, (pos,))?;

        for i in 0..10 {
            let hscore = &global_state.high_scores.get(i);
            let (name, score) = match hscore {
                None => ("---".to_string(), "---".to_string()),
                Some(hscore) => (hscore.name.to_string(), hscore.score.to_string()),
            };
            let len = name.len();
            let fonts = &global_state.fonts;
            let ndx = ggez::graphics::Text::new((i.to_string(), fonts.font, fonts.medium));
            let mut pos = Point2::new(WIDTH / 4.0, 178.0 + (i as f32) * 38.0);
            ggez::graphics::draw(ctx, &ndx, (pos,))?;

            let name = ggez::graphics::Text::new((name, fonts.font, fonts.medium));
            pos.x = pos.x + 20.0;
            ggez::graphics::draw(ctx, &name, (pos,))?;

            let score = ggez::graphics::Text::new((score, fonts.font, fonts.medium));
            pos.x = pos.x + (len as f32) * 8.0 + 20.0;
            ggez::graphics::draw(ctx, &score, (pos,))?;
        }

        let shift = 4.0 * EXIT_LEN;
        let pos = Point2::new((WIDTH / 2.0) - shift, HEIGHT - 50.0);
        ggez::graphics::draw(ctx, &self.exit, (pos,))
    }
}
