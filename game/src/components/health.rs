use std::time::Duration;

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

/// Allow to set max health point of object so it can regenerate to it
#[derive(Component)]
pub struct MaxHealth(f32);

impl MaxHealth {
    pub fn new(max_health: f32) -> Self {
        Self(max_health)
    }

    pub fn max_health(&self) -> f32 {
        self.0
    }
}

/// Tells entity with health fully regenerate on next update
#[derive(Component)]
pub struct RegenerateOneTimeToFull;

/// Adds fully healed entity with specified health
#[derive(Bundle)]
pub struct HealthBundle {
    health: Health,
    max_health: MaxHealth,
}

impl HealthBundle {
    pub fn new(health: f32) -> Self {
        Self {
            health: Health::new(health),
            max_health: MaxHealth::new(health),
        }
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

/// Mark that entity should not take damage
#[derive(Component, Clone)]
pub struct Immortality;

/// Mark that entity should not take damage
#[derive(Component, Clone)]
pub struct TimedImmortality(Timer);

impl std::ops::Deref for TimedImmortality {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TimedImmortality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Bundle)]
pub struct TimedImmortalityBundle {
    mark: Immortality,
    timer: TimedImmortality,
}

impl TimedImmortalityBundle {
    pub fn new(time: f32) -> Self {
        let timer = Timer::new(Duration::from_secs_f32(time), TimerMode::Once);

        Self {
            mark: Immortality,
            timer: TimedImmortality(timer),
        }
    }
}
