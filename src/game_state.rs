use crate::parser::{Scene, Story};

#[derive(Debug)]
pub struct GameState {
    scene: Scene,
    hp: u64,
    // inventory: String
}

impl GameState{
    pub fn new(scene: Scene, hp: u64) -> Self {
        Self { scene, hp }
    }
    pub fn display_scene(&self){
        println!("{}\n{}\n", self.scene.title(), self.scene.text());
        // afficher choix / ending
    }
    pub fn display_inventary(&self){
        println!("ceci est le joli inventaire");
    }
    pub fn display_hp(&self){
        println!("pov la vie : {}", self.hp);
    }
    pub fn choose(&mut self, choice: usize, story: &Story){
        self.scene = story.scenes().as_ref().expect("aie").iter().find(|s| s.id() == self.scene.choices().expect("msg").get(choice).expect("msg").next()).expect("msg").clone();
        self.display_scene();
    }
}