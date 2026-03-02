
use crate::{game_state::GameState, parser::Story};

pub trait GameCommand {
    fn execute(
        &self,
        story: &Story,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError>;
}


#[derive(Debug)]
pub enum CommandOutcome {
    Continue,
    Quit
}

#[derive(Debug)]
pub enum GameError {
    InvalidChoice,
    NoSuchAction,
    InternalError(String),
}


pub struct LookCommand;

impl GameCommand for LookCommand {
    fn execute(
            &self,
            story: &Story,
            state: &mut GameState,
        ) -> Result<CommandOutcome, GameError> {
        let _ = state.display_scene();
        Ok(CommandOutcome::Continue)
    }
}


pub struct ChooseCommand{
    pub choice: i32
}

impl GameCommand for ChooseCommand {
    fn execute(
            &self,
            story: &Story,
            state: &mut GameState,
        ) -> Result<CommandOutcome, GameError> {
        state.choose(self.choice, story);
        Ok(CommandOutcome::Continue)
    }
}


pub struct InventoryCommand;

impl GameCommand for InventoryCommand {
    fn execute(
            &self,
            story: &Story,
            state: &mut GameState,
        ) -> Result<CommandOutcome, GameError> {
        let _ = state.display_inventary();
        Ok(CommandOutcome::Continue)
    }
}

pub struct StatusCommand;

impl GameCommand for StatusCommand {
    fn execute(
            &self,
            story: &Story,
            state: &mut GameState,
        ) -> Result<CommandOutcome, GameError> {
        let _ = state.display_hp();
        let _ = state.display_scene();
        Ok(CommandOutcome::Continue)
    }
}


pub struct QuitCommand;

impl GameCommand for QuitCommand {
    fn execute(
            &self,
            story: &Story,
            state: &mut GameState,
        ) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Quit)
    }
}
