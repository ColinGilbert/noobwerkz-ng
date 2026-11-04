#![allow(unused)]
use crate::ecs_components_common::*;
use crate::index_types::*;
use crate::scene3d::Characters3DContext;
use rapier3d::math::Orientation;
use winit::dpi::Position;
use zero_ecs::*;

#[derive(Default)]
pub struct CharacterTypeIndexComponent {
    pub val: Characters3DContextIndex,
}

#[derive(Default)]
pub struct CharacterInstanceIndexComponent {
    pub val: Character3DIndex,
}

pub struct CharactersContextsResource<'a> {
    pub val: &'a mut Vec<Characters3DContext>,
}

impl<'a> CharactersContextsResource<'a> {
    pub fn new(contexts: &'a mut Vec<Characters3DContext>) -> Self {
        Self { val: contexts }
    }
}

#[entity]
#[derive(Default)]
pub struct CharacterEntity {
    pub type_idx: CharacterTypeIndexComponent,
    pub instance_idx: CharacterInstanceIndexComponent,
    pub position: PositionComponent,
    pub velocity: VelocityComponent,
    pub orientation: OrientationComponent,
    pub speed: SpeedComponent,
    pub path: PathComponent,
    pub character_component: CharacterComponent, // SOmetimes we need to filter on this
}

ecs_world!(CharacterEntity);

#[system_for_each(World)]
fn update_positions(
    type_idx: &CharacterTypeIndexComponent,
    instance_idx: &CharacterInstanceIndexComponent,
    position: &mut PositionComponent,
    velocity: &mut VelocityComponent,
    orientation: &mut OrientationComponent,
    speed: &mut SpeedComponent,
    path: &mut PathComponent,
    dt: &DeltaTimeResource,
    all_characters_contexts: &mut CharactersContextsResource,
) {
    let characters_ctx = &mut all_characters_contexts.val[type_idx.val.get() as usize];
    let character = &mut characters_ctx.characters[instance_idx.val];
    let mut position_temp = position.val;
    let mut orientation_temp = orientation.val;
    let mut velocity_temp = velocity.val;
    position_temp += velocity_temp * dt.val;
    position.val = position_temp;
    // character.
    character.common.position = position_temp;
    character.common.orientation = orientation_temp;
    let mut speed_temp = speed.val;
    let path = &mut path.val;
}
