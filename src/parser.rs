use serde::{Deserialize, Serialize};
use serde_yaml;
use crate::story_error::StoryError;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    start_scene: String,
    initial_hp: i64,
    scenes: Option<Vec<Scene>>
}

impl Story{
    pub fn start_scene(&self) -> Scene {
        let scenes = self.scenes.as_ref().expect("aie");
        scenes.iter().find(|s| s.id == self.start_scene).expect("msg").clone()
    }
    pub fn initial_hp(&self) -> i64 {
        self.initial_hp.clone()
    }
    pub fn scenes(&self) -> Option<Vec<Scene>> {
        self.scenes.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    id: String,
    title: String,
    text: String,
    found_item: Option<String>,
    hp_delta: Option<i64>,
    choices: Option<Vec<Choice>>,
    ending: Option<String>
}

impl Scene{
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }
    pub fn found_item(&self) -> Option<String> {
        self.found_item.clone()
    }
    pub fn hp_delta(&self) -> Option<i64> {
        self.hp_delta.clone()
    }
    pub fn choices(&self) -> Option<Vec<Choice>> {
        self.choices.clone()
    }
    pub fn ending(&self) -> Option<String> {
        self.ending.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    label: String,
    next: String,
    required_item: Option<String>
}

impl Choice{
    pub fn label(&self) -> String {
        self.label.clone()
    }
    pub fn next(&self) -> String {
        self.next.clone()
    }
    pub fn required_item(&self) -> Option<String> {
        self.required_item.clone()
    }
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

