use serde::{Deserialize, Serialize};


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


