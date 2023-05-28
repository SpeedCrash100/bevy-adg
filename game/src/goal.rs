use bevy::prelude::*;
use rand::Rng;

use crate::math::RotateAroundZ;

pub const GOAL_REACH_RANGE: f32 = 100.0;
pub const GOAL_UI_DISTANCE: f32 = 50.0;

#[derive(Resource, Clone, Debug)]
pub struct Goal {
    pub position: Vec2,
    pub points: u32,
}

impl Goal {
    pub fn player_reached(&mut self) {
        self.set_new_goal();
        self.gain_point();
    }

    pub fn reset(&mut self) {
        self.position = Vec2::ZERO;
        self.points = 0;
        self.set_new_goal();
    }

    fn set_new_goal(&mut self) {
        let mut rng = rand::thread_rng();

        let direction = rng.gen_range(-180.0..180.0);
        let range = rng.gen_range(4000.0..10000.0);

        let translate = Vec2::X.rotate_z(direction) * range;

        self.position += translate;
    }

    fn gain_point(&mut self) {
        self.points += 1;
    }
}

impl Default for Goal {
    fn default() -> Self {
        let mut zero_goal = Self {
            position: Vec2::ZERO,
            points: 0,
        };

        zero_goal.set_new_goal();

        zero_goal
    }
}

pub struct GoalChangeEvent(Vec2);

impl GoalChangeEvent {
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }

    pub fn position(&self) -> &Vec2 {
        &self.0
    }
}
