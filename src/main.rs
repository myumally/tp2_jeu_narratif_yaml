use tp2_jeu_narratif_yaml::{game_loop::start_game, story_error::StoryError};

fn main() -> Result<(), StoryError> {
    start_game()
}