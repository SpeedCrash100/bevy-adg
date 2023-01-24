use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, ContactForceEventThreshold};

/// Mark that component is dead
#[derive(Component)]
pub struct Dead;

/// Health component
#[derive(Component, Default)]
pub struct Health(f32);

impl Health {
    pub fn new(health: f32) -> Self {
        Self(health)
    }

    #[inline(always)]
    pub fn health(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    pub fn alive(&self) -> bool {
        0.0 < self.health()
    }

    #[inline(always)]
    pub fn damage(&mut self, damage: f32) {
        if self.health() < 0.0 {
            return;
        }

        self.0 -= damage;
    }
}

/// Mark that entity takes damage by collision with other objects
#[derive(Component)]
pub struct CollisionDamage;

#[derive(Bundle)]
pub struct CollisionDamageBundle {
    mark: CollisionDamage,
    events: ActiveEvents,
    threshold: ContactForceEventThreshold,
}

impl CollisionDamageBundle {
    pub fn new() -> Self {
        Self {
            mark: CollisionDamage,
            events: ActiveEvents::CONTACT_FORCE_EVENTS,
            threshold: ContactForceEventThreshold(100_000.0),
        }
    }
}
