use std::io;

use crate::{game_state::GameState, parse_command::parse_command, parser::{Story, story_from_file}, story_error::StoryError};

pub fn start_game() -> Result<(), StoryError> {
    let cfg: Story = story_from_file("story.yaml")?;
    let mut state = GameState::new(cfg.start_scene(), cfg.initial_hp());
    state.display_scene();
    game_loop(&cfg, &mut state);
    Ok(())
}

fn game_loop(story: &Story, state: &mut GameState){
    let mut quit = false;
    while !quit{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur");
        match parse_command(&input).expect("erreur pour parse la commande").execute(story, state).expect("erreur pour exec la commande"){
            crate::command::CommandOutcome::Quit => quit = true,
            crate::command::CommandOutcome::GameOver => {
                println!("Skill issue you're dead");
                quit = true;
            }
            _ => quit = false
        }
    }
    
}