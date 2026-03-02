use serde::{Deserialize, Serialize};
use serde_yaml;
use crate::story_error::StoryError;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    start_scene: String,
    initial_hp: u64,
    scenes: Option<Vec<Scene>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    id: String,
    title: String,
    text: String,
    choices: Option<Vec<Choice>>,
    ending: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    label: String,
    next: String
}

pub fn story_from_file(file: &str) -> Result<Story, StoryError>{
    let raw = std::fs::File::open(file)?;
    let cfg: Story = serde_yaml::from_reader(raw)?;
    let start_scene = &cfg.start_scene;
    let scenes = cfg.scenes.as_ref().ok_or(StoryError::UnvalidStartScene)?;
    if !scenes.iter().any(|s| &s.id == start_scene){
        return Err(StoryError::UnvalidStartScene);
    }
    let mut ids = HashSet::new();
    for scene in scenes {
        if !ids.insert(&scene.id) {
            return Err(StoryError::NonUniqueIdScene);
        }
    }
    for scene in scenes {
        if let Some(choices) = &scene.choices{
            for choice in choices{
                if !ids.iter().any(|id| id == &&choice.next) {
                    return Err(StoryError::UnexistantChoice);
                }
            }
        }
    }
    Ok(cfg)
}

