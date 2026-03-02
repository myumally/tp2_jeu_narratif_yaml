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

impl Story{
    pub fn start_scene(&self) -> Scene {
        let scenes = self.scenes.as_ref().expect("aie");
        scenes.iter().find(|s| s.id == self.start_scene).expect("msg").clone()
    }
    pub fn initial_hp(&self) -> u64 {
        self.initial_hp.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    id: String,
    title: String,
    text: String,
    choices: Option<Vec<Choice>>,
    ending: Option<String>
}

impl Scene{
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

