use ggez::glam::{vec2, Vec2};
use rand::Rng;
use std::ops::{Add, AddAssign, Mul};

pub struct Ball {
    pub current_location: Vec2,
    previous_location: Vec2,
    acceleration: Vec2,

    pub size: f32,
}

pub const BORDER_RADIUS: f32 = 80.0;

impl Ball {
    pub fn new() -> Ball {
        Ball {
            current_location: vec2(50., 0.),
            previous_location: vec2(50., 0.),
            acceleration: Vec2::ZERO,
            size: rand::thread_rng().gen_range(1.0..7.0),
        }
    }

    pub fn add_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration.add_assign(acceleration);
    }

    pub fn keep_in_circle(&mut self) {
        let length = self.current_location.length();

        if (length + self.size) > BORDER_RADIUS {
            self.current_location =
                self.current_location.normalize_or_zero() * (BORDER_RADIUS - self.size);
        }
    }

    pub fn update(&mut self, delta: f32) {
        let velocity = self.current_location - self.previous_location;
        let velocity = velocity * 0.999;

        self.previous_location = self.current_location;

        self.current_location =
            self.current_location + velocity + self.acceleration * (delta * delta);

        self.acceleration = Vec2::ZERO;
    }
}
