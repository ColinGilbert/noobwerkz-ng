use crate::index_types::*;

pub struct ItemCommon {
    pub uid: Item3DUIDIndex,
    pub item_type_uid: Item3DTypeUIDIndex,
    pub health: f32,
    pub status: ItemStatus,
}

pub enum ItemStatus {
    WithPlayer(ItemStatusWithPlayerType),
    WithNPC(ItemStatusWithNPCType),
    InWorld(ItemStatusInWorldType),
}

pub struct ItemStatusWithPlayerType {
    pub player_uid: Player3DUIDIndex,
    pub equipped: bool,
}

pub struct ItemStatusWithNPCType {
    pub npc_uid: NPC3DUIDIndex,
    pub equipped: bool,
}

pub struct ItemStatusInWorldType {
    pub position: glam::Vec3,
    // pub orientation: glam::Quat,
}

pub struct ItemType {
    pub equippable: bool,
    pub weight: f32,
    pub dims: glam::Vec3,
    pub name: String,
}