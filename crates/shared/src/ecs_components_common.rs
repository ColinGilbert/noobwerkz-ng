#![allow(unused)]
use std::collections::VecDeque;

use zero_ecs::*;

#[derive(Default)]
pub struct PositionComponent
{
    pub val: glam::Vec3,
}

#[derive(Default)]
pub struct VelocityComponent {
    pub val: glam::Vec3,
}

#[derive(Default)]
pub struct OrientationComponent {
    pub val: glam::Quat,
}

#[derive(Default)]
pub struct SpeedComponent {
    pub val: f32,
}

#[derive(Default)]
pub struct PathComponent {
    pub val: VecDeque<glam::Vec3>,
}

#[derive(Default)]
pub struct CharacterComponent;

pub struct DeltaTimeResource {
    pub val: f32,
}
