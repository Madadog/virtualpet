use bevy::prelude::*;

/// Marker struct for entities controllable by RtsController
#[derive(Component)]
pub struct NavigationDestination(pub Option<Vec2>);

pub enum Action {
    Idle,
    Moving,
    Using,
}

#[derive(Event)]
pub struct MoveCommand {
    destination: Vec2,
    entity: Entity,
}
