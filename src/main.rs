use tp2_jeu_narratif_yaml::{parser::{Story, story_from_file}, story_error::StoryError};

fn main() -> Result<(), StoryError> {
    let cfg: Story = story_from_file("story.yaml")?;
    println!("{cfg:?}");
    Ok(())
}