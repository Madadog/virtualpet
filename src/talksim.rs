use bevy::prelude::*;

pub struct Talksim {
    actors: Vec<Actor>,
    dialogue: Vec<Dialogue>,
    
}

pub struct Actor {
    entity: Entity,
    alignment: (f32, f32),
    name: String,
    sprite_index: usize,
}

pub enum Dialogue {
    NewString(String),
    AppendString(String),
    Pause(f32),
    SetAlignment(String, (f32, f32)),
    SetSprite(String, usize),
}
