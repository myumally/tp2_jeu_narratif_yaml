use crate::{command::{CommandOutcome, GameError}, parser::{Scene, Story}};

#[derive(Debug)]
pub struct GameState {
    scene: Scene,
    hp: i64,
    // inventory: String
}

impl GameState{
    pub fn new(scene: Scene, hp: i64) -> Self {
        Self { scene, hp }
    }
    pub fn display_scene(&self){
        println!("{}\n{}\n", self.scene.title(), self.scene.text());
        if let Some(choices) = self.scene.choices(){
            for choice in choices{
                println!("{}", choice.label());
            }
        }
        if let Some(ending) = self.scene.ending(){
            println!("{}", ending);
        }
    }
    pub fn display_inventary(&self){
        println!("ceci est le joli inventaire");
    }
    pub fn display_hp(&self){
        println!("{} hp left", self.hp);
    }
    pub fn choose(&mut self, n: usize, story: &Story) -> Result<CommandOutcome, GameError>{
        if let Some(scenes) = story.scenes().as_ref(){
            if let Some(choices) = self.scene.choices(){
                if let Some(choice) = choices.get(n){
                    if let Some(next) = scenes.iter().find(|s| s.id() == choice.next()){
                        self.scene = next.clone();
                        self.display_scene();
                        if let Some(delta) = self.scene.hp_delta(){
                            self.hp += delta;
                            if self.hp <= 0 {
                                return Ok(CommandOutcome::GameOver);
                            }
                        }
                        return Ok(CommandOutcome::Continue);
                    }
                }
            }
        }
        println!("invalid choice");
        return Err(GameError::InvalidChoice);

        // self.scene = story.scenes().as_ref().expect("aie").iter().find(|s| s.id() == self.scene.choices().expect("msg").get(choice).expect("msg").next()).expect("msg").clone();
    }
}