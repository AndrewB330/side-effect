use bevy::ecs::schedule::States;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Copy, Default)]
pub enum GameWorldState {
    #[default]
    None,
    GameWorld,
}
