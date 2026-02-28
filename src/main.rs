use serde_yaml;
use tp2_jeu_narratif_yaml::parser::{Story};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw = std::fs::File::open("story.yaml")?;
    let cfg: Story = serde_yaml::from_reader(raw)?;
    println!("{cfg:?}");
    Ok(())
}