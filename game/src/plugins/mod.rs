/// Adds astroid spawning/despawning in world
pub mod asteroid;
/// Adds background to game
pub mod background;
/// Adds despawn options for objects by using [DespawnOn](crate::components::common::DespawnOn) mark
pub mod despawn;
/// Adds hud to screen
pub mod hud;
/// Handles most of InGame state logic(Despawn, Reset, etc)
pub mod ingame;
/// Adds living(entity with [Health]) processing: adds [Dead] marks so you can process what to do when object is dead
pub mod living;
/// Adds particles processing
pub mod particle;
/// Adds pause game
pub mod pause;
/// Adds physic to world: no gravity enviroments and ability to sum external forces of children
pub mod physics;
/// Adds player ship and controls for it
pub mod player;
/// Adds respawn menu
pub mod respawn;
/// Adds ship processing system
pub mod ship;
/// Adds weapons processing system
pub mod weapon;

/// Adds goal to the game and score system
pub mod goal;

/// Adds displayable information on entities with its health
pub mod healthdisplay;
