use super::GlobalState;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub enum StateKind {
    // EnterHighScores,
    // GameOver,
    HighScores(u32),
    PaddleSelect(u32),
    // Play,
    // Serve,
    Start(u32),
    // Victory,
}

impl StateKind {
    fn to_string(&self) -> String {
        match self {
            StateKind::HighScores(_) => "highscores".to_string(),
            StateKind::PaddleSelect(_) => "paddleselect".to_string(),
            StateKind::Start(_) => "start".to_string(),
        }
    }
}

pub trait State {
    fn enter(&mut self, params: StateKind);
    fn exit(&self);
    fn update(
        &mut self,
        global_state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult<Option<StateKind>>;
    fn render(&self, global_state: &GlobalState, ctx: &mut Context) -> GameResult<()>;
}

pub struct StartState {
    highlighted: u32,
    high_scores: u32,
    title: ggez::graphics::Text,
    start: ggez::graphics::Text,
    high_score: ggez::graphics::Text,
}

pub struct StateMachine {
    pub states: HashMap<String, Box<dyn State>>,
    current: Option<String>,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            states: HashMap::new(),
            current: None,
        }
    }

    pub fn change(&mut self, params: StateKind) {
        match &self.current {
            Some(key) => {
                let current_state = &self.states[key];
                current_state.exit()
            }
            None => (),
        }
        let key = params.to_string();
        let current = self.states.get_mut(&key).unwrap();
        current.enter(params);
        self.current = Some(key)
    }

    pub fn render(&self, global_state: &GlobalState, ctx: &mut Context) -> GameResult<()> {
        match &self.current {
            Some(key) => {
                let current_state = &self.states[key];
                current_state.render(global_state, ctx)
            },
            None => Ok(())
        }
    }
    pub fn update(
        &mut self,
        global_state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult {
        match &mut self.current {
            Some(key) => {
                let current_state = self.states.get_mut(key).unwrap();
                let next_state = current_state.update(global_state, ctx);
                match next_state {
                    Ok(Some(new_state)) => {
                        self.change(new_state);
                        Ok(())
                    },
                    Ok(None) => Ok(()),
                    Err(_) => panic!("")
                }
            },
            None => Ok(())
        }
    }
}

// function StateMachine:update(dt)
//     self.current:update(dt)
// end

// function StateMachine:render()
//     self.current:render()
// end

mod start_state;