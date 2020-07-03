use std::collections::HashMap;
use std::collections::HashSet;
use std::path;

use ggez::audio;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use ggez::event::{KeyCode, KeyMods};

use cgmath::Point2;

mod state;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

enum SoundKind {
    PaddleHit,
    Score,
    WallHit,
    Confirm,
    Select,
    NoSelect,
    BrickHit1,
    BrickHit2,
    Hurt,
    Victory,
    Recover,
    HighScore,
    Pause,
    Music,
}

impl SoundKind {
    fn to_string(self) -> String {
        match self {
            SoundKind::PaddleHit => "PaddleHit".to_string(),
            SoundKind::Score => "Score".to_string(),
            SoundKind::WallHit => "WallHit".to_string(),
            SoundKind::Confirm => "Confirm".to_string(),
            SoundKind::Select => "Select".to_string(),
            SoundKind::NoSelect => "NoSelect".to_string(),
            SoundKind::BrickHit1 => "BrickHit1".to_string(),
            SoundKind::BrickHit2 => "BrickHit2".to_string(),
            SoundKind::Hurt => "Hurt".to_string(),
            SoundKind::Victory => "Victory".to_string(),
            SoundKind::Recover => "Recover".to_string(),
            SoundKind::HighScore => "HighScore".to_string(),
            SoundKind::Pause => "Pause".to_string(),
            SoundKind::Music => "Music".to_string(),
        }
    }
}

type Sounds = HashMap<String, audio::Source>;
type Keys = HashSet<KeyCode>;

pub struct Fonts {
    font: ggez::graphics::Font,
    small: f32,
    medium: f32,
    large: f32,
}

pub struct HighScore {
    name: String,
    score: u32,
}

pub struct GlobalState {
    sounds: Sounds,
    fonts: Fonts,
    state_machine: state::StateMachine,
    high_scores: Vec<HighScore>,
    keys_pressed: Keys,
}

impl GlobalState {
    fn new(ctx: &mut Context) -> GameResult<GlobalState> {
        let mut sounds = HashMap::new();
        sounds.insert(
            SoundKind::PaddleHit.to_string(),
            audio::Source::new(ctx, "/sounds/paddle_hit.wav")?,
        );
        sounds.insert(
            SoundKind::Score.to_string(),
            audio::Source::new(ctx, "/sounds/score.wav")?,
        );
        sounds.insert(
            SoundKind::WallHit.to_string(),
            audio::Source::new(ctx, "/sounds/wall_hit.wav")?,
        );
        sounds.insert(
            SoundKind::Confirm.to_string(),
            audio::Source::new(ctx, "/sounds/confirm.wav")?,
        );
        sounds.insert(
            SoundKind::Select.to_string(),
            audio::Source::new(ctx, "/sounds/select.wav")?,
        );
        sounds.insert(
            SoundKind::NoSelect.to_string(),
            audio::Source::new(ctx, "/sounds/no-select.wav")?,
        );
        sounds.insert(
            SoundKind::BrickHit1.to_string(),
            audio::Source::new(ctx, "/sounds/brick-hit-1.wav")?,
        );
        sounds.insert(
            SoundKind::BrickHit2.to_string(),
            audio::Source::new(ctx, "/sounds/brick-hit-2.wav")?,
        );
        sounds.insert(
            SoundKind::Hurt.to_string(),
            audio::Source::new(ctx, "/sounds/hurt.wav")?,
        );
        sounds.insert(
            SoundKind::Victory.to_string(),
            audio::Source::new(ctx, "/sounds/victory.wav")?,
        );
        sounds.insert(
            SoundKind::Recover.to_string(),
            audio::Source::new(ctx, "/sounds/recover.wav")?,
        );
        sounds.insert(
            SoundKind::HighScore.to_string(),
            audio::Source::new(ctx, "/sounds/high_score.wav")?,
        );
        sounds.insert(
            SoundKind::Pause.to_string(),
            audio::Source::new(ctx, "/sounds/pause.wav")?,
        );
        sounds.insert(
            SoundKind::Music.to_string(),
            audio::Source::new(ctx, "/sounds/music.wav")?,
        );

        let font = ggez::graphics::Font::new(ctx, "/fonts/font.ttf")?;
        let fonts = Fonts {
            font,
            small: 16.0,
            medium: 24.0,
            large: 48.0,
        };

        let mut states = state::StateMachine::new();
        let start_state = state::StartState::new(&fonts);
        let high_score_state = state::HighScoreState::new(&fonts);
        states
            .states
            .insert("start".to_string(), Box::new(start_state));
        states
            .states
            .insert("highscores".to_string(), Box::new(high_score_state));

        // switch to start screen
        states.change(state::StateKind::Start);

        let state = GlobalState {
            fonts,
            sounds,
            state_machine: states,
            high_scores: vec![],
            keys_pressed : HashSet::new()
        };
        Ok(state)
    }

    fn display_fps(&mut self, ctx: &mut Context) -> GameResult<()> {
        let color = ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0);
        let fps = format!("FPS: {:.2}", timer::fps(ctx));
        let text = ggez::graphics::Text::new((fps, self.fonts.font, self.fonts.small));
        let pos = Point2::new(30.0, 30.0);
        ggez::graphics::draw(ctx, &text, (pos, color))
    }
}

impl ggez::event::EventHandler for GlobalState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state_machine.update(&mut self.sounds, &self.keys_pressed, ctx)?;
        self.keys_pressed = HashSet::new();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // TODO: draw backgroundd
        ggez::graphics::clear(ctx, [40.0 / 255.0, 45.0 / 255.0, 52.0 / 255.0, 1.0].into());

        self.state_machine.render(self, ctx)?;
        self.display_fps(ctx)?;
        ggez::graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) {
        let _ = self.keys_pressed.insert(key);
    }

}

fn main() {
    let resource_dir = path::PathBuf::from("./resources");
    let mut c = ggez::conf::Conf::new();
    c.window_mode.width = WIDTH;
    c.window_mode.height = HEIGHT;

    let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("breakout", "vinz")
        .conf(c)
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("BreakOut")
                .vsync(false),
        )
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
    // let rng = rand::thread_rng();

    let global_state = &mut GlobalState::new(ctx).unwrap();

    ggez::event::run(ctx, event_loop, global_state).unwrap();
}
