use bevy::prelude::*;
use components::ui::MainWindow;
use physic_objects::prelude::*;
use states::GameState;

#[macro_use]
extern crate derive_builder;

mod components;
mod entity;
mod math;
mod plugins;
mod random;
mod stages;
mod states;

pub fn run() {
    let mut app = App::new();

    // base compnents
    app.add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicObjectPlugin)
        .add_plugin(plugins::background::BackgroundPlugin)
        .add_plugin(plugins::physics::PhysicsPlugin)
        .add_plugin(plugins::living::LivingPlugin)
        .add_plugin(plugins::despawn::DespawnPlugin)
        .add_plugin(plugins::asteroid::AsteroidsPlugin)
        .add_plugin(plugins::ship::ShipPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::weapon::WeaponPlugin)
        .add_plugin(plugins::hud::HudPlugin)
        .add_plugin(plugins::ingame::InGamePlugin)
        .add_plugin(plugins::pause::PausePlugin)
        .add_plugin(plugins::respawn::RespawnPlugin)
        .add_plugin(plugins::particle::ParticlePlugin)
        .add_startup_system(insert_marker_in_window);

    // Debug only components
    if cfg!(feature = "debug_hp") {
        app.add_plugin(plugins::healthdisplay::HealthDisplayPlugin);
    }

    // Start up
    app.run();
}

fn insert_marker_in_window(mut commands: Commands, wnds: Query<Entity, With<Window>>) {
    let wnd = wnds.single();
    commands.entity(wnd).insert(MainWindow);
}
