use std::collections::HashSet;
use noobwerkz_shared::{physics3d_context::*, index_types::*, item3d_common::*};
use crate::{index_types::*, server_player3d::*, server_npc3d::*, server_item3d::*};
pub struct ServerScene3D {
    pub physics: Physics3DContext,
    pub player_uids_to_local: Player3DUIDsMap<ServerPlayer3DIndex>,
    pub players: ServerPlayers3D<ServerPlayer3D>,
    pub players_index_freelist: ServerPlayers3DSet,
    pub npc_uids_to_local: NPC3DUIDsMap<ServerNPC3DIndex>,
    pub nps: ServerNPCs3D<ServerNPC3D>,
    pub npcs_index_freelist: ServerNPCs3DSet,
    pub item_uids_to_local: Item3DUIDsMap<HashSet<ServerItem3DIndex>>,
    pub items: ServerItem3Ds<ServerItem3D>,
    pub items_index_freelist: ServerItems3DSet,
    pub item_types: Item3DTypeUIDsMap<ItemType>,
    // TODO: Navmesh, NPCs, player characters, buildings, items, actions
}