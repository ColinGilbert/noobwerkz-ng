use crate::{character3d_common::Character3DCommon, index_types::Player3DUIDIndex};

pub struct Player3DCommon {
    pub character_common: Character3DCommon,
    pub uid: Player3DUIDIndex,
}